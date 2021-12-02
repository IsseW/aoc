use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

use crate::helpers::NodeId;

fn parse(input: &str) -> (HashSet<NodeId>, HashMap<NodeId, HashMap<NodeId, u32>>) {
    let mut places = HashSet::new();
    let mut distances: HashMap<NodeId, HashMap<NodeId, u32>> = HashMap::new();
    input.lines().for_each(|line| {
        if let Some((from, _, to, _, distance)) = line.split_whitespace().collect_tuple() {
            let (from, to) = (NodeId::new(from), NodeId::new(to));
            let distance = u32::from_str_radix(distance, 10).unwrap();

            places.insert(from);
            places.insert(to);
            if let Some(from) = distances.get_mut(&from) {
                from.insert(to, distance);
            } else {
                let mut t = HashMap::new();
                t.insert(to, distance);
                distances.insert(from, t);
            }
            if let Some(to) = distances.get_mut(&to) {
                to.insert(from, distance);
            } else {
                let mut t = HashMap::new();
                t.insert(from, distance);
                distances.insert(to, t);
            }
        }
    });
    (places, distances)
}

pub fn solution_1(input: &str) -> String {
    let (places, distances) = parse(input);

    places
        .iter()
        .permutations(places.len())
        .map(|path| {
            path.iter()
                .zip(path.iter().skip(1))
                .map(|(&from, &to)| {
                    distances
                        .get(from)
                        .and_then(|from| from.get(to))
                        .unwrap_or(&0xFFFF)
                })
                .sum::<u32>()
        })
        .min()
        .unwrap()
        .to_string()
}

pub fn solution_2(input: &str) -> String {
    let (places, distances) = parse(input);

    places
        .iter()
        .permutations(places.len())
        .map(|path| {
            path.iter()
                .zip(path.iter().skip(1))
                .map(|(&from, &to)| {
                    distances
                        .get(from)
                        .and_then(|from| from.get(to))
                        .unwrap_or(&0xFFFF)
                })
                .sum::<u32>()
        })
        .max()
        .unwrap()
        .to_string()
}
