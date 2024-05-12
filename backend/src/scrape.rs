use std::collections::HashMap;

use anyhow::Result;
use geo::{Coord, LineString};
use osm_reader::OsmID;
use rstar::primitives::GeomWithData;
use rstar::RTree;
use utils::Tags;

use crate::amenity::Amenity;
use crate::graph::{
    AmenityID, Graph, Intersection, IntersectionID, IntersectionLocation, Road, RoadID,
};
use crate::route::Router;

struct ReadAmenities {
    amenities: Vec<Amenity>,
}

impl utils::osm2graph::OsmReader for ReadAmenities {
    fn node(&mut self, id: osm_reader::NodeID, pt: Coord, tags: Tags) {
        self.amenities.extend(Amenity::maybe_new(
            &tags,
            OsmID::Node(id),
            pt.into(),
            AmenityID(self.amenities.len()),
        ));
    }

    fn way(
        &mut self,
        id: osm_reader::WayID,
        node_ids: &Vec<osm_reader::NodeID>,
        node_mapping: &HashMap<osm_reader::NodeID, Coord>,
        tags: &Tags,
    ) {
        self.amenities.extend(Amenity::maybe_new(
            tags,
            OsmID::Way(id),
            // TODO Centroid
            node_mapping[&node_ids[0]].into(),
            AmenityID(self.amenities.len()),
        ));
    }

    // TODO Are there amenities as relations?
}

pub fn scrape_osm(input_bytes: &[u8]) -> Result<Graph> {
    info!("Parsing {} bytes of OSM data", input_bytes.len());

    let mut amenities = ReadAmenities {
        amenities: Vec::new(),
    };
    let graph = utils::osm2graph::Graph::new(
        input_bytes,
        |tags| {
            tags.has("highway")
                && !tags.is("highway", "proposed")
                && !tags.is("area", "yes")
                && !tags.is("foot", "no")
        },
        &mut amenities,
    )?;

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
    for a in &mut amenities.amenities {
        a.point = graph.mercator.pt_to_mercator(a.point.into()).into();
    }

    snap_amenities(&mut roads, &amenities.amenities);

    let mut points = Vec::new();
    for i in &intersections {
        points.push(IntersectionLocation::new(i.point.into(), i.id));
    }
    let closest_intersection = RTree::bulk_load(points);

    let router = Router::new(&roads);

    Ok(Graph {
        roads,
        intersections,
        mercator: graph.mercator,
        closest_intersection,
        boundary_polygon: graph.boundary_polygon,

        amenities: amenities.amenities,
        router,
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
