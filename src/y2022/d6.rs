use iterslide::SlideIterator;
use itertools::Itertools;

fn solve(input: &str, n: usize) -> String {
	let (i, _) = input.chars().slide(n).enumerate().find(|(_, f)| {
		f.iter().all_unique()
	}).unwrap();
	(i + n).to_string()
}

pub fn solution_1(input: &str) -> String {
	solve(input, 4)
}

pub fn solution_2(input: &str) -> String {
	solve(input, 14)
}
