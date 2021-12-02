use crate::helpers::NodeId;
use hashbrown::HashMap;
use itertools::Itertools;

#[derive(Debug)]
pub struct Node<'a> {
    pub name: &'a str,
    pub connections: HashMap<NodeId, i32>,
}

fn parse(input: &str) -> HashMap<NodeId, Node> {
    let mut map: HashMap<NodeId, Node> = HashMap::new();
    input.lines().for_each(|line| {
        let mut split = line.split_whitespace();
        let name = split.next().unwrap();
        split.advance_by(1).unwrap();
        let gain = match split.next().unwrap() {
            "lose" => -1,
            _ => 1,
        } * i32::from_str_radix(split.next().unwrap(), 10).unwrap();
        split.advance_by(6).unwrap();
        let o_name = split.next().unwrap().trim_end_matches(|c| c == '.');
        let id = NodeId::new(name);
        let o_id = NodeId::new(o_name);
        if let Some(node) = map.get_mut(&id) {
            node.connections.insert(o_id, gain);
        } else {
            let mut m = HashMap::new();
            m.insert(o_id, gain);
            map.insert(
                id,
                Node {
                    name,
                    connections: m,
                },
            );
        }
    });
    map
}

fn find_optimal(map: &HashMap<NodeId, Node>) -> i32 {
    map.keys()
        .permutations(map.len())
        .map(|permutation| {
            let mut sum = 0;
            for i in 0..permutation.len() as i32 {
                let node = &map[permutation[i as usize]];
                let left =
                    permutation[(i - 1 + permutation.len() as i32) as usize % permutation.len()];
                let right = permutation[(i + 1) as usize % permutation.len()];
                let gain = node.connections.get(left).unwrap_or(&0)
                    + node.connections.get(right).unwrap_or(&0);
                sum += gain;
            }
            sum
        })
        .max()
        .unwrap()
}

pub fn solution_1(input: &str) -> String {
    let map = parse(input);
    find_optimal(&map).to_string()
}

pub fn solution_2(input: &str) -> String {
    let mut map = parse(input);
    map.insert(
        NodeId::new("Isse"),
        Node {
            name: "Isse",
            connections: HashMap::new(),
        },
    );
    find_optimal(&map).to_string()
}
