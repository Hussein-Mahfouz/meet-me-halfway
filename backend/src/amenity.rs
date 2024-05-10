use geo::Point;
use geojson::{Feature, Geometry};
use osm_reader::OsmID;
use utils::{Mercator, Tags};

use crate::graph::AmenityID;

pub struct Amenity {
    pub id: AmenityID,
    pub osm_id: OsmID,
    pub point: Point,
    pub kind: String,
    pub name: Option<String>,
}

impl Amenity {
    pub fn maybe_new(tags: &Tags, osm_id: OsmID, point: Point, id: AmenityID) -> Option<Self> {
        let kind = Self::is_amenity(tags)?;
        Some(Self {
            id,
            osm_id,
            point,
            name: tags.get("name").cloned(),
            kind,
        })
    }

    pub fn to_gj(&self, mercator: &Mercator) -> Feature {
        let mut f = Feature::from(Geometry::from(&mercator.to_wgs84(&self.point)));
        f.set_property("amenity_kind", self.kind.clone());
        f.set_property("osm_id", self.osm_id.to_string());
        if let Some(ref name) = self.name {
            f.set_property("name", name.clone());
        }
        f
    }

    /// Determines if this OSM object should count as some kind of useful commercial amenity.
    /// Returns the category.
    fn is_amenity(tags: &Tags) -> Option<String> {
        tags.get("amenity").or_else(|| tags.get("shop")).cloned()
    }
}
