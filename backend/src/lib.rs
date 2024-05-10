#[macro_use]
extern crate log;

use std::sync::Once;

use geo::Coord;
use geojson::GeoJson;
use serde::Deserialize;
use wasm_bindgen::prelude::*;

use self::graph::Graph;

mod amenity;
mod find;
mod graph;
mod route;
mod scrape;

static START: Once = Once::new();

// TODO Rename
#[wasm_bindgen]
pub struct MapModel {
    graph: Graph,
}

#[wasm_bindgen]
impl MapModel {
    /// Call with bytes of an osm.pbf or osm.xml string
    #[wasm_bindgen(constructor)]
    pub fn new(input_bytes: &[u8]) -> Result<MapModel, JsValue> {
        // Panics shouldn't happen, but if they do, console.log them.
        console_error_panic_hook::set_once();
        START.call_once(|| {
            console_log::init_with_level(log::Level::Info).unwrap();
        });

        Ok(MapModel {
            graph: Graph::new(input_bytes).map_err(err_to_js)?,
        })
    }

    /// Return a polygon covering the world, minus a hole for the boundary, in WGS84
    #[wasm_bindgen(js_name = getInvertedBoundary)]
    pub fn get_inverted_boundary(&self) -> Result<String, JsValue> {
        self.graph.get_inverted_boundary().map_err(err_to_js)
    }

    #[wasm_bindgen(js_name = getBounds)]
    pub fn get_bounds(&self) -> Vec<f64> {
        let b = &self.graph.mercator.wgs84_bounds;
        vec![b.min().x, b.min().y, b.max().x, b.max().y]
    }

    #[wasm_bindgen(js_name = findPOIs)]
    pub fn find_pois(&self, input: JsValue) -> Result<String, JsValue> {
        let req: CalculateRequest = serde_wasm_bindgen::from_value(input)?;
        let results = find::find_pois(&self.graph, req);
        Ok(serde_json::to_string(&results).map_err(err_to_js)?)
    }

    #[wasm_bindgen(js_name = routesTo)]
    pub fn routes_to(&mut self, input: JsValue) -> Result<String, JsValue> {
        let req: RouteRequest = serde_wasm_bindgen::from_value(input)?;
        let end = self
            .graph
            .closest_intersection
            .nearest_neighbor(&x_y(self.graph.mercator.pt_to_mercator(Coord {
                x: req.point[0],
                y: req.point[1],
            })))
            .unwrap()
            .data;

        let mut features = Vec::new();
        for person in req.people {
            let start = self
                .graph
                .closest_intersection
                .nearest_neighbor(&x_y(self.graph.mercator.pt_to_mercator(Coord {
                    x: person.home[0],
                    y: person.home[1],
                })))
                .unwrap()
                .data;
            features.extend(
                self.graph
                    .router
                    .route(&self.graph, start, end)
                    .map_err(err_to_js)?,
            );
        }
        Ok(serde_json::to_string(&GeoJson::from(features)).map_err(err_to_js)?)
    }
}

#[derive(Deserialize)]
pub struct CalculateRequest {
    people: Vec<Person>,
}

#[derive(Deserialize)]
pub struct RouteRequest {
    people: Vec<Person>,
    point: [f64; 2],
}

#[derive(Deserialize)]
pub struct Person {
    name: String,
    home: [f64; 2],
    #[serde(rename = "maxTimeMinutes")]
    max_time_minutes: u64,
}

fn err_to_js<E: std::fmt::Display>(err: E) -> JsValue {
    JsValue::from_str(&err.to_string())
}

fn x_y(c: Coord) -> [f64; 2] {
    [c.x, c.y]
}
