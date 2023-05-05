use hashbrown::HashMap;
use petgraph::prelude::UnGraphMap;

#[derive(PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
struct Node(u8);

fn parse(input: &str) -> (UnGraphMap<Node, u32>, Vec<(Node, u32)>) {
	let mut graph = UnGraphMap::new();
	let mut rates = Vec::new();
	let mut node_c = 0u8;
	let mut nodes = HashMap::new();
	let mut node = |key: String| -> Node {
		*nodes.entry(key).or_insert_with(|| {
			let i = node_c;
			node_c += 1;
			Node(i)
		})
	};

	let start = node("AA".to_string());

	for line in input.lines() {
		let (id, rate, rest): (String, u32, String) = strp::scan!(line => "Valve {} has flow rate={}; {}");
		let connected = rest.strip_prefix("tunnels lead to valve").or(rest.strip_prefix("tunnel leads to valve")).unwrap();
		let connected = connected.strip_prefix('s').unwrap_or(connected).trim_start();
		let n = node(id);
		if rate > 0 {
			rates.push((n, rate));
		}
		for conn in connected.split(", ") {
			graph.add_edge(n, node(conn.to_string()), 1.0);

		}
	}
	rates.sort_by_key(|n| n.0);

	let mut ngraph = UnGraphMap::new();

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
struct Game<const L: usize> {
	released: u32,
	open: u128,
	positions: [(Node, u32); L],
}

fn search_game<const L: usize>(game: Game<L>, map: &UnGraphMap<Node, u32>, rates: &HashMap<Node, u32>) -> u32 {
	let (i, (position, timeleft)) = game.positions.iter().enumerate().max_by_key(|(_, (_, t))| t).unwrap();
	if *timeleft == 0 {
		return game.released;
	}
	map.edges(*position).filter(|(_, e, cost)| (game.open & (1 << e.0)) == 0 && 1 + **cost < *timeleft).map(|(_, e, cost)| {
		let mut game = game.clone();
		let (_, time) = game.positions[i];
		let time = time - (1 + cost);
		game.positions[i] = (e, time);
		game.open |= 1 << e.0;

		game.released += rates.get(&e).unwrap_or(&0) * time;

		search_game(game, map, rates)
	}).max().unwrap_or(game.released)
}

fn solve<const L: usize>(input: &str, time: u32) -> u32 {
	let (graph, rates) = parse(input);

	let start = Node(0);
	search_game(Game {
		released: 0,
		open: 1,
		positions: [(start, time); L],
	}, &graph, &rates.into_iter().collect())
}


pub fn solution_1(input: &str) -> String {
	solve::<1>(input, 30).to_string()
}

pub fn solution_2(input: &str) -> String {
	solve::<2>(input, 26).to_string()
}
