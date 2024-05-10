use anyhow::Result;
use geo::{LineString, Point, Polygon};
use geojson::{Feature, Geometry};
use rstar::{primitives::GeomWithData, RTree};
use utils::Mercator;

use crate::amenity::Amenity;

// This is only for walking
pub struct Graph {
    pub roads: Vec<Road>,
    pub intersections: Vec<Intersection>,
    // All geometry stored in worldspace, including rtrees
    pub mercator: Mercator,
    pub closest_intersection: RTree<IntersectionLocation>,
    pub boundary_polygon: Polygon,

    pub amenities: Vec<Amenity>,
}

pub type IntersectionLocation = GeomWithData<[f64; 2], IntersectionID>;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct RoadID(pub usize);
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct IntersectionID(pub usize);
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct AmenityID(pub usize);

pub struct Road {
    pub id: RoadID,
    pub src_i: IntersectionID,
    pub dst_i: IntersectionID,
    pub linestring: LineString,

    pub amenities: Vec<AmenityID>,
}

pub struct Intersection {
    pub id: IntersectionID,
    pub point: Point,
    pub roads: Vec<RoadID>,
}

impl Graph {
    /// Call with bytes of an osm.pbf or osm.xml string
    pub fn new(input_bytes: &[u8]) -> Result<Graph> {
        // TODO make a method there
        crate::scrape::scrape_osm(input_bytes)
    }

    /// Return a polygon covering the world, minus a hole for the boundary, in WGS84
    pub fn get_inverted_boundary(&self) -> Result<String> {
        let (boundary, _) = self.mercator.to_wgs84(&self.boundary_polygon).into_inner();
        let polygon = Polygon::new(
            LineString::from(vec![
                (180.0, 90.0),
                (-180.0, 90.0),
                (-180.0, -90.0),
                (180.0, -90.0),
                (180.0, 90.0),
            ]),
            vec![boundary],
        );
        let f = Feature::from(Geometry::from(&polygon));
        let out = serde_json::to_string(&f)?;
        Ok(out)
    }

    pub fn roads_per_intersection(&self, i: IntersectionID) -> impl Iterator<Item = &Road> {
        self.intersections[i.0]
            .roads
            .iter()
            .map(|r| &self.roads[r.0])
    }

    pub fn find_edge(&self, i1: IntersectionID, i2: IntersectionID) -> &Road {
        // TODO Store lookup table
        for r in &self.intersections[i1.0].roads {
            let road = &self.roads[r.0];
            if road.src_i == i2 || road.dst_i == i2 {
                return road;
            }
        }
        panic!("no road from {i1:?} to {i2:?} or vice versa");
    }
}

impl Road {
    pub fn other_side(&self, i: IntersectionID) -> IntersectionID {
        if self.src_i == i {
            self.dst_i
        } else {
            // TODO Assumes the input is one of the two, and assumes no self-loops
            self.src_i
        }
    }
}
