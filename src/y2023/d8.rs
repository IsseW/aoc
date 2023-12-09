use hashbrown::HashMap;
use num::integer::lcm;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(u16);

impl NodeId {
    pub fn new(s: &str) -> Self {
        Self(s.bytes().fold(0, |mut state, b| {
            state <<= 5;
            state |= (b - b'A') as u16;
            state
        }))
    }

    fn ends_with(&self, b: u8) -> bool {
        let b = b - b'A';
        self.0 & (b as u16) == (b as u16)
    }
}

pub fn solution_1(input: &str) -> String {
    let (dirs, nodes) = input.split_once("\n\n").unwrap();
    let mut instructions = dirs.bytes().map(|c| if c == b'R' { 1 } else { 0 }).cycle();
    let nodes: HashMap<_, _> = nodes
        .lines()
        .map(|line| {
            (
                NodeId::new(&line[0..3]),
                [NodeId::new(&line[7..10]), NodeId::new(&line[12..15])],
            )
        })
        .collect();

    let mut current_node = NodeId::new("AAA");
    let end = NodeId::new("ZZZ");

    let mut i = 0;
    while current_node != end {
        let instr = instructions.next().unwrap();
        i += 1;
        current_node = nodes[&current_node][instr];
    }

    i.to_string()
}

fn gcd(mut n: u128, mut m: u128) -> u128 {
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}
pub fn solution_2(input: &str) -> String {
    let (dirs, node_input) = input.split_once("\n\n").unwrap();
    let instruction_count = dirs.len();
    let instructions = dirs
        .bytes()
        .map(|c| if c == b'R' { 1 } else { 0 })
        .enumerate()
        .cycle();
    let mut nodes = vec![[NodeId(0), NodeId(0)]; u16::MAX as usize];
    let mut start_nodes = Vec::new();
    for node in node_input.lines().map(|line| {
        (
            NodeId::new(&line[0..3]),
            [NodeId::new(&line[7..10]), NodeId::new(&line[12..15])],
        )
    }) {
        if node.0.ends_with(b'A') {
            start_nodes.push(node.0);
        }
        nodes[node.0 .0 as usize] = node.1;
    }

    type Num = u64;

    start_nodes
        .into_iter()
        .map(|n| {
            let mut current_node = n;
            let mut i = 0 as Num;
            let mut instructions = instructions.clone();
            let mut last_node = None;
            let (start, end) = loop {
                let (idx, instr) = instructions.next().unwrap();
                if current_node.ends_with(b'Z') {
                    if let Some(j) = last_node {
                        break (j, i);
                    } else {
                        last_node = Some(i);
                    }
                }
                i += 1;
                current_node = nodes[current_node.0 as usize][instr];
            };

            end - start
        })
        .fold(1, |acc, x| lcm(acc, x))
        .to_string()
}
