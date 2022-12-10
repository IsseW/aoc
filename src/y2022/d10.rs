use crate::helpers::{self, Grid, GridIndexMut};

enum Instr {
	Noop,
	Add(i32),
}

impl Instr {
	fn cycles(&self) -> i32 {
		match self {
			Instr::Noop => 1,
			Instr::Add(_) => 2,
		}
	}

	fn exec(&self, x: &mut i32) {
		match self {
			Instr::Noop => {},
			Instr::Add(val) => *x += val,
		}
	}
}

fn parse<'a>(input: &'a str) -> impl Iterator<Item = Instr> + 'a {
	input.lines().map(|line| {
		if line == "noop" {
			Instr::Noop
		} else {
			let (instr, val) = line.split_once(' ').unwrap();
			assert_eq!(instr, "addx");
			let val = val.parse().unwrap();
			
			Instr::Add(val)
		}
	})
}

pub fn solution_1(input: &str) -> String {
	let mut x = 1i32;
	let mut cycle = 0i32;
	let mut sum = 0;
	for instr in parse(input) {
		let last_cycle = cycle;
		cycle += instr.cycles();
		let get_cycle = |c: i32| (((c + 20) / 40) * 40 - 20);
		if get_cycle(last_cycle) != get_cycle(cycle) {
			sum += get_cycle(cycle) * x;
		}
		instr.exec(&mut x);
	}

	sum.to_string()
}

pub fn solution_2(input: &str) -> String {
	let mut screen = Grid::<bool>::new(40, 6);
	
	let mut x = 1i32;
	let mut cycle = 0i32;
	for instr in parse(input) {
		let last_cycle = cycle;
		cycle += instr.cycles();
		for i in last_cycle..cycle {
			if (i % 40).abs_diff(x) <= 1 {
				*screen.get_mut(i % 40, i / 40).unwrap() = true;
			}
		}
		instr.exec(&mut x);
	}

	screen.parse_word()
}
