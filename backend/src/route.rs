use std::cell::RefCell;

use anyhow::{bail, Result};
use fast_paths::{FastGraph, InputGraph, PathCalculator};
use geojson::{Feature, Geometry};
use utils::NodeMap;

use crate::graph::{Graph, IntersectionID, Road};

pub struct Router {
    node_map: NodeMap<IntersectionID>,
    ch: FastGraph,
    path_calc: RefCell<PathCalculator>,
}

impl Router {
    pub fn new(roads: &Vec<Road>) -> Self {
        let mut input_graph = InputGraph::new();
        let mut node_map = NodeMap::new();

        for road in roads {
            let cost = road.get_cost().as_millis() as usize;
            let node1 = node_map.get_or_insert(road.src_i);
            let node2 = node_map.get_or_insert(road.dst_i);

            input_graph.add_edge(node1, node2, cost);
            input_graph.add_edge(node2, node1, cost);
        }
        input_graph.freeze();
        let ch = fast_paths::prepare(&input_graph);

        let path_calc = RefCell::new(fast_paths::create_calculator(&ch));

        Self {
            node_map,
            ch,
            path_calc,
        }
    }

    pub fn route(
        &self,
        graph: &Graph,
        start: IntersectionID,
        end: IntersectionID,
    ) -> Result<Vec<Feature>> {
        if start == end {
            bail!("start = end");
        }
        let start = self.node_map.get(start).unwrap();
        let end = self.node_map.get(end).unwrap();

        let Some(path) = self.path_calc.borrow_mut().calc_path(&self.ch, start, end) else {
            bail!("No path");
        };

        // TODO Ideally glue together one LineString
        let mut features = Vec::new();
        for pair in path.get_nodes().windows(2) {
            let i1 = self.node_map.translate_id(pair[0]);
            let i2 = self.node_map.translate_id(pair[1]);
            let road = graph.find_edge(i1, i2);
            features.push(Feature::from(Geometry::from(
                &graph.mercator.to_wgs84(&road.linestring),
            )));
        }
        Ok(features)
    }
}
