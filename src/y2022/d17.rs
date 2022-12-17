use crate::helpers::{self, Grid, GridIndex, CollectGridFlat};

fn parse(input: &str) -> impl ExactSizeIterator<Item = isize> + '_ + Clone {
	input.bytes().map(|c| {
		match c as char {
			'<' => -1,
			'>' => 1,
			c => panic!("Unexpected char {c:?}"),
		}
	})
}

lazy_static::lazy_static! {
	static ref SHAPES: Vec<Grid<bool>> = {
		vec![
			[true, true, true, true].into_iter().collect_grid_f(4, 1),
			[
				false, true, false, 
				true, true, true,
				false, true, false,
			].into_iter().collect_grid_f(3, 3),
			[
				true, true, true,
				false, false, true,
				false, false, true,
			].into_iter().collect_grid_f(3, 3),
			[true, true, true, true].into_iter().collect_grid_f(1, 4),
			[true, true, true, true].into_iter().collect_grid_f(2, 2),
		]
	};
}

fn sim_piece(i: usize, jets: &mut impl Iterator<Item = isize>, grid: &mut Grid<bool>, highest: &mut usize) -> (usize, usize) {
	let shape = &SHAPES[i % SHAPES.len()];
	let mut pos = (2usize, *highest + 3);
	loop {
		if let Some(push) = jets.next() {
			let next_pos = (pos.0.saturating_add_signed(push), pos.1);
			if !grid.collides(shape, next_pos) {
				pos = next_pos;
			}
		}
		if let Some(y) = pos.1.checked_sub(1) {
			let next_pos = (pos.0, y);
			if grid.collides(shape, next_pos) {
				break;
			} else {
				pos = next_pos;
			}
		} else {
			break;
		}
	}
	*highest = (*highest).max(pos.1 + shape.height());
	grid.or(shape, pos);
	pos
}

fn simulate(jets: impl Iterator<Item = isize> + Clone, num: usize) -> usize {
	let mut jets = jets.cycle();
	let height = SHAPES.iter().map(|shape| shape.height()).sum::<usize>() * (num + 3) / 4;
	let mut grid = Grid::new(7, height);

	let mut highest = 0;
	for i in 0..num {
		sim_piece(i, &mut jets, &mut grid, &mut highest);
	}
	highest
}

pub fn solution_1(input: &str) -> String {
	simulate(parse(input), 2022).to_string()
}

pub fn solution_2(input: &str) -> String {
	let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
	let n = 1000000000000;
	let input = parse(input);

	let cycle_len = input.len() * SHAPES.len();
	
	let mut jets = input.clone().cycle();

	let height = SHAPES.iter().map(|shape| shape.height()).sum::<usize>() * cycle_len * 1000000;
	let mut grid = Grid::new(7, height);

	let mut highest = 0;
	let mut first_cycle = Vec::new();
	for i in 0..cycle_len {
		first_cycle.push(sim_piece(i, &mut jets, &mut grid, &mut highest).0);
	}
	let (cycle_height, cycle_len) = (1..).find_map(|k| {
		if k % 10000 == 0 {
			dbg!(k * cycle_len);
		}
		let mut fits_first = true;
		let last_highest = highest;
		for i in 0..cycle_len {
			let first = first_cycle[i];
			let i = i + k * cycle_len;
			let res = sim_piece(i, &mut jets, &mut grid, &mut highest);
			if highest > height {
				panic!()
			}
			if fits_first && res.0 != first {
				fits_first = false;
			}
		}
		fits_first.then_some((last_highest, k * cycle_len))
	}).unwrap();

	let rest = simulate(input.cycle(), n % cycle_len);

	(cycle_height * (n / cycle_len) + rest).to_string()
}
