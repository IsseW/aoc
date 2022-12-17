use itertools::Itertools;
use vek::Vec2;

use crate::helpers::{Grid, straight};

#[derive(Copy, Clone, PartialEq, Default)]
enum Cell {
	#[default]
	Air,
	Rock,
	Sand,
}

fn parse(input: &str, mut min: Vec2<u32>, mut max: Vec2<u32>) -> (Grid<Cell>, Vec2<u32>) {
	let lines = input.lines().map(|line| {
		line.split(" -> ").map(|coord| {
			let (x, y) = coord.split_once(',').unwrap();
			let (x, y): (u32, u32) = (x.parse().unwrap(), y.parse().unwrap());
			let res = Vec2::new(x, y);
			min = res.map2(min, |a, b| a.min(b));
			max = res.map2(max, |a, b| a.max(b));
			res
		}).collect_vec()
	}).collect_vec();

	let size = max - min + 1;
	let mut grid = Grid::new(size.x as usize, size.y as usize + 1);

	for line in lines {
		line.into_iter().reduce(|a, b| {
			for (x, y) in straight(a.into_tuple(), b.into_tuple()) {
				let x = (x - min.x) as usize;
				let y = (y - min.y) as usize;
				grid[(x, y)] = Cell::Rock;
			}
			b
		});
	}

	(grid, min)
}

pub fn solution_1(input: &str) -> String {
	let (mut grid, min) = parse(input, Vec2::new(u32::MAX, 0), Vec2::zero());
	let sand_pos = ((500 - min.x) as usize, (0 - min.y) as usize);
	let mut sand_count = 0;
	'sim: loop {
		let mut sand_pos = sand_pos;
		if matches!(grid[sand_pos], Cell::Sand) {
			break 'sim;
		}
		loop {
			let check_pos = (sand_pos.0, sand_pos.1 + 1);
			if check_pos.1 >= grid.height() {
				break 'sim;
			}
			if matches!(grid[check_pos], Cell::Air) {
				sand_pos = check_pos;
				continue;
			}
			if let Some(x) = sand_pos.0.checked_sub(1) {
				let check_pos = (x, sand_pos.1 + 1);
				if matches!(grid[check_pos], Cell::Air) {
					sand_pos = check_pos;
					continue;
				}
			} else {
				break 'sim;
			}
			let check_pos = (sand_pos.0 + 1, sand_pos.1 + 1);
			if check_pos.0 >= grid.width() {
				break 'sim;
			}
			if matches!(grid[check_pos], Cell::Air) {
				sand_pos = check_pos;
				continue;
			}
			grid[sand_pos] = Cell::Sand;
			break;
		}
		sand_count += 1;
	}
	sand_count.to_string()
}

pub fn solution_2(input: &str) -> String {
	let (mut grid, min) = parse(input, Vec2::new(328, 0), Vec2::new(672, 0));
	let sand_pos = ((500 - min.x) as usize, (0 - min.y) as usize);
	grid[sand_pos] = Cell::Sand;
	let mut sand_count = 1;

	let center = sand_pos.0;
	for y in 1..grid.height() {
		// Will always be false
		let mut last_center = false;
		let mut last_right = false;
		for x in (center - y)..=(center + y) {
			let right = matches!(grid[(x + 1, y - 1)], Cell::Sand);
			if matches!(grid[(x, y)], Cell::Air) && (last_center || last_right || right) {
				grid[(x, y)] = Cell::Sand;
				sand_count += 1;
			}
			last_center = last_right;
			last_right = right;
		}
	}

	sand_count.to_string()
}
