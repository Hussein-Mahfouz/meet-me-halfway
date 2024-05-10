use std::collections::HashMap;

use anyhow::Result;
use geo::{Coord, LineString};
use osm_reader::{Element, OsmID};
use rstar::primitives::GeomWithData;
use rstar::RTree;
use utils::Tags;

use crate::amenity::Amenity;
use crate::graph::{
    AmenityID, Graph, Intersection, IntersectionID, IntersectionLocation, Road, RoadID,
};

pub fn scrape_osm(input_bytes: &[u8]) -> Result<Graph> {
    info!("Parsing {} bytes of OSM data", input_bytes.len());
    // This doesn't use osm2graph's helper, because it needs to scrape more things from OSM
    let mut node_mapping = HashMap::new();
    let mut highways = Vec::new();
    let mut amenities = Vec::new();
    osm_reader::parse(input_bytes, |elem| match elem {
        Element::Node {
            id, lon, lat, tags, ..
        } => {
            let pt = Coord { x: lon, y: lat };
            node_mapping.insert(id, pt);

            let tags = tags.into();
            amenities.extend(Amenity::maybe_new(
                &tags,
                OsmID::Node(id),
                pt.into(),
                AmenityID(amenities.len()),
            ));
        }
        Element::Way {
            id,
            mut node_ids,
            tags,
            ..
        } => {
            let tags: Tags = tags.into();

            amenities.extend(Amenity::maybe_new(
                &tags,
                OsmID::Way(id),
                // TODO Centroid
                node_mapping[&node_ids[0]].into(),
                AmenityID(amenities.len()),
            ));

            if tags.has("highway")
                && !tags.is("highway", "proposed")
                && !tags.is("area", "yes")
                && !tags.is("foot", "no")
            {
                // TODO This sometimes happens from Overpass?
                let num = node_ids.len();
                node_ids.retain(|n| node_mapping.contains_key(n));
                if node_ids.len() != num {
                    warn!("{id} refers to nodes outside the imported area");
                }
                if node_ids.len() >= 2 {
                    highways.push(utils::osm2graph::Way { id, node_ids, tags });
                }
            }
        }
        // TODO Amenity relations?
        Element::Relation { .. } => {}
        Element::Bounds { .. } => {}
    })?;

    info!("Splitting {} ways into edges", highways.len());
    let graph = utils::osm2graph::Graph::from_scraped_osm(node_mapping, highways);

    // Copy all the fields
    let intersections: Vec<Intersection> = graph
        .intersections
        .into_iter()
        .map(|i| Intersection {
            id: IntersectionID(i.id.0),
            point: i.point,
            roads: i.edges.into_iter().map(|e| RoadID(e.0)).collect(),
        })
        .collect();

    // Add in a bit
    let mut roads = graph
        .edges
        .into_iter()
        .map(|e| Road {
            id: RoadID(e.id.0),
            src_i: IntersectionID(e.src.0),
            dst_i: IntersectionID(e.dst.0),
            linestring: e.linestring,
            amenities: Vec::new(),
        })
        .collect();
    for a in &mut amenities {
        a.point = graph.mercator.pt_to_mercator(a.point.into()).into();
    }

    snap_amenities(&mut roads, &amenities);

    let mut points = Vec::new();
    for i in &intersections {
        points.push(IntersectionLocation::new(i.point.into(), i.id));
    }
    let closest_intersection = RTree::bulk_load(points);

    Ok(Graph {
        roads,
        intersections,
        mercator: graph.mercator,
        closest_intersection,
        boundary_polygon: graph.boundary_polygon,

        amenities,
    })
}

type EdgeLocation = GeomWithData<LineString, RoadID>;

fn snap_amenities(roads: &mut Vec<Road>, amenities: &Vec<Amenity>) {
    let closest_road = RTree::bulk_load(
        roads
            .iter()
            .map(|r| EdgeLocation::new(r.linestring.clone(), r.id))
            .collect(),
    );

    for amenity in amenities {
        if let Some(r) = closest_road.nearest_neighbor(&amenity.point) {
            roads[r.data.0].amenities.push(amenity.id);
        }
    }
}
