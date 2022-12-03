use itertools::Itertools;

use crate::helpers;

fn value_of(c: char) -> u64 {
	1 << match c {
		'a'..='z' => (c as u8 - b'a') as u64,
		'A'..='Z' => (c as u8 - b'A') as u64 + 26,
		_ => unreachable!(),
	}
}

fn convert_to_bitset(s: &str) -> u64 {
	s.chars().fold(0, |acc, c| acc | value_of(c))
}

pub fn solution_1(input: &str) -> String {
	input.lines().map(|line| {
		let line = line.trim();
		let half = line.len() / 2;
		let (a, b) = (&line[..half], &line[half..]);

		let (a, b) = (convert_to_bitset(a), convert_to_bitset(b));

		(a & b).trailing_zeros() as u64 + 1
	}).sum::<u64>().to_string()
}

pub fn solution_2(input: &str) -> String {
	input.lines().chunks(3).into_iter().map(|group| {
		group.map(|line| convert_to_bitset(line)).reduce(|a, b| a & b).unwrap().trailing_zeros() as u64 + 1
	}).sum::<u64>().to_string()
}
