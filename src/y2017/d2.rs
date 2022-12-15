use itertools::Itertools;

use crate::helpers;

pub fn solution_1(input: &str) -> String {
	input.lines().map(|line| {
		let numbers = line.split_whitespace().map(|n| n.parse::<u32>().unwrap()).collect_vec();

		numbers.iter().max().unwrap() - numbers.iter().min().unwrap()
	}).sum::<u32>().to_string()
}

pub fn solution_2(input: &str) -> String {
	input.lines().filter_map(|line| {
		let numbers = line.split_whitespace().map(|n| n.parse::<u32>().unwrap()).collect_vec();

		numbers.iter().permutations(2).find_map(|v| (v[0] % v[1] == 0).then_some(v[0] / v[1]))
	}).sum::<u32>().to_string()
}
