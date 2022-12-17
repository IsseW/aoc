use std::{fmt::Display, iter::once};

use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use petgraph::prelude::UnGraphMap;

#[derive(PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
struct Node(u16);

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.0 as u8 as char, (self.0 >> 8) as u8 as char)
    }
}

fn node(id: &str) -> Option<Node> {
	let (a, b): (u8, u8) = id.bytes().collect_tuple()?;

	Some(Node(a as u16 + ((b as u16) << 8)))
}

fn parse(input: &str) -> (UnGraphMap<Node, u32>, Vec<(Node, u32)>) {
	let mut graph = UnGraphMap::new();
	let mut rates = Vec::new();
	for line in input.lines() {
		let (id, rate, rest): (String, u32, String) = strp::scan!(line => "Valve {} has flow rate={}; {}");
		let connected = rest.strip_prefix("tunnels lead to valve").or(rest.strip_prefix("tunnel leads to valve")).unwrap();
		let connected = connected.strip_prefix('s').unwrap_or(connected).trim_start();
		let n = node(&id).unwrap();
		if rate > 0 {
			rates.push((n, rate));
		}
		for conn in connected.split(", ") {
			graph.add_edge(n, node(conn).unwrap(), 1.0);

		}
	}
	rates.sort_by_key(|n| n.0);

	let mut ngraph = UnGraphMap::new();

	let start = node("AA").unwrap();
	for node in rates.iter().map(|(n, _)| *n).chain(std::iter::once(start)) {
		let costs = petgraph::algo::dijkstra(
			&graph,
			node,
			None,
			|(_, _, cost)|*cost,
		);
		for (n, _) in rates.iter() {
			let cost = costs[n];
			// Cut off values to make this faster. Should work on input from aoc.
			if cost > 3.0 {
				continue;
			}
			ngraph.add_edge(node, *n, cost as u32);
		}
	}
	(ngraph, rates)
}


#[derive(Clone)]
struct Game {
	released: u32,
	open: HashSet<Node>,
	positions: Vec<(Node, u32)>,
}

fn search_game(game: Game, map: &UnGraphMap<Node, u32>, rates: &HashMap<Node, u32>) -> u32 {
	let (i, (position, timeleft)) = game.positions.iter().enumerate().max_by_key(|(_, (_, t))| t).unwrap();
	if *timeleft == 0 {
		return game.released;
	}
	map.edges(*position).filter(|(_, e, cost)| !game.open.contains(&e) && 1 + **cost < *timeleft).map(|(_, e, cost)| {
		let mut game = game.clone();
		let (_, time) = game.positions[i];
		let time = time - (1 + cost);
		game.positions[i] = (e, time);
		game.open.insert(e);

		game.released += rates.get(&e).unwrap_or(&0) * time;

		search_game(game, map, rates)
	}).max().unwrap_or(game.released)
}

fn solve(input: &str, time: u32, count: usize) -> u32 {
	let (graph, rates) = parse(input);

	let start = node("AA").unwrap();
	search_game(Game {
		released: 0,
		open: once(start).collect(),
		positions: vec![(start, time); count],
	}, &graph, &rates.into_iter().collect())
}


pub fn solution_1(input: &str) -> String {
	solve(input, 30, 1).to_string()
}

pub fn solution_2(input: &str) -> String {
	solve(input, 26, 2).to_string()
}
