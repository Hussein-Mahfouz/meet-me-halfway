use std::collections::{BinaryHeap, HashMap, HashSet};
use std::time::Duration;

use geo::{Coord, Point};
use serde::Serialize;
use utils::PriorityQueueItem;

use crate::graph::{AmenityID, Graph, IntersectionID};
use crate::{x_y, CalculateRequest, Person};

#[derive(Serialize)]
pub struct POI {
    pub osm_url: String,
    pub point: Point,
    pub kind: String,
    pub name: Option<String>,

    /// (Name, cost in seconds)
    pub times_per_person: Vec<(String, u64)>,
}

pub fn find_pois(graph: &Graph, req: CalculateRequest) -> Vec<POI> {
    let mut pois: HashMap<AmenityID, POI> = HashMap::new();

    let num_people = req.people.len();
    for person in req.people {
        for (a, cost) in get_costs(graph, &person) {
            let amenity = &graph.amenities[a.0];
            pois.entry(a)
                .or_insert_with(|| POI {
                    osm_url: amenity.osm_id.to_string(),
                    point: graph.mercator.to_wgs84(&amenity.point),
                    kind: amenity.kind.clone(),
                    name: amenity.name.clone(),
                    times_per_person: Vec::new(),
                })
                .times_per_person
                .push((person.name.clone(), cost.as_secs()));
        }
    }

    // Only keep results that everyone can reach
    pois.into_values()
        .filter(|poi| poi.times_per_person.len() == num_people)
        .collect()
}

fn get_costs(graph: &Graph, person: &Person) -> HashMap<AmenityID, Duration> {
    let start = graph
        .closest_intersection
        .nearest_neighbor(&x_y(graph.mercator.pt_to_mercator(Coord {
            x: person.home[0],
            y: person.home[1],
        })))
        .unwrap()
        .data;
    let limit = Duration::from_secs(60 * person.max_time_minutes);

    let mut visited: HashSet<IntersectionID> = HashSet::new();
    let mut cost_per_poi: HashMap<AmenityID, Duration> = HashMap::new();
    let mut queue: BinaryHeap<PriorityQueueItem<Duration, IntersectionID>> = BinaryHeap::new();

    queue.push(PriorityQueueItem::new(Duration::ZERO, start));

    while let Some(current) = queue.pop() {
        if visited.contains(&current.value) {
            continue;
        }
        visited.insert(current.value);
        if current.cost > limit {
            continue;
        }

        for road in graph.roads_per_intersection(current.value) {
            let total_cost = current.cost + road.get_cost();

            for a in &road.amenities {
                cost_per_poi.insert(*a, total_cost);
            }

            queue.push(PriorityQueueItem::new(
                total_cost,
                road.other_side(current.value),
            ));
        }
    }

    cost_per_poi
}
