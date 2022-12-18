use itertools::Itertools;


fn parse(input: &str) -> impl Iterator<Item = u32> + '_ + Clone {
	input.lines().map(str::parse).map(Result::unwrap)
}

pub fn solution_1(input: &str) -> String {
	let (a, b) = parse(input).tuple_combinations().find(|(a, b)| a + b == 2020).unwrap();
	(a * b).to_string()
}

pub fn solution_2(input: &str) -> String {
	let (a, b, c) = parse(input).tuple_combinations().find(|(a, b, c)| a + b + c == 2020).unwrap();
	(a * b * c).to_string()
}
