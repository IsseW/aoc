use hashbrown::HashMap;


fn parse(input: &str) -> impl Iterator<Item = (usize, usize)> + '_ {
	input.lines().map(|line| {
		let vertical = &line[0..7];
		let horizontal = &line[7..];
		
		(
			horizontal.chars().fold(0, |acc, c| (acc << 1) | match c {
				'R' => 1,
				'L' => 0,
				_ => panic!("Unexpected character: {c}"),
			}),
			vertical.chars().fold(0, |acc, c| (acc << 1) | match c {
				'B' => 1,
				'F' => 0,
				_ => panic!("Unexpected character: {c}"),
			}),
		)
	})
}

pub fn solution_1(input: &str) -> String {
	parse(input).map(|(x, y)| y * 8 + x).max().unwrap().to_string()
}

pub fn solution_2(input: &str) -> String {
	let map = parse(input).map(|(x, y)| (y * 8 + x, (x, y))).collect::<HashMap<_, _>>();
	(0..).find(|id| map.get(id).is_none() && map.get(&id.saturating_sub(1)).is_some() && map.get(&(id + 1)).is_some()).unwrap().to_string()
}
