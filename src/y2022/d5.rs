use std::collections::BTreeMap;

use itertools::Itertools;

use crate::helpers::{self, StrUtil};

fn parse_stacks(input: &str) -> Vec<Vec<char>> {
	let mut stacks: BTreeMap<usize, Vec<char>> = BTreeMap::new();
	for line in input.lines().rev().skip(1) {
		for (i, c) in line.find_enclosures('[', ']').enumerate() {
			let offset = c.as_ptr() as usize - line.as_ptr() as usize;
			stacks.entry(offset).or_insert(Vec::new()).push(c.chars().next().unwrap());
		}
	}
	stacks.values().cloned().collect()
}

fn parse_code<'a>(input: &'a str) -> impl Iterator<Item = (usize, usize, usize)> + 'a {
	input.lines().map(|line| {
		let (n, from, to): (u32, u32, u32) = strp::scan!(line => "move {} from {} to {}");
		(n as usize, from as usize - 1, to as usize - 1)
	})
}

pub fn solution_1(input: &str) -> String {
	let (stacks, code) = input.split_once("\n\n").unwrap();
	let mut stacks = parse_stacks(stacks);
	
	parse_code(code).for_each(|(n, from, to)| {
		for _ in 0..n {
			if let Some(c) = stacks[from].pop() {
				stacks[to].push(c);
			}
		}
	});

	stacks.into_iter().filter_map(|vec| vec.last().copied()).collect()
}

pub fn solution_2(input: &str) -> String {
	let (stacks, code) = input.split_once("\n\n").unwrap();
	let mut stacks = parse_stacks(stacks);
	
	parse_code(code).for_each(|(n, from, to)| {
		let r = stacks[from].len().saturating_sub(n);
		let r = stacks[from].drain(r..).collect_vec();
		stacks[to].extend(r);
	});

	stacks.into_iter().filter_map(|vec| vec.last().copied()).collect()
}
