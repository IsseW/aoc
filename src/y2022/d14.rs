use itertools::Itertools;
use vek::Vec2;

use crate::helpers::{self, Grid, LineIter, GridLinearSlice, GridLinearSliceMut, straight};

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
			let (x, y): (u32, u32) = strp::scan!(coord => "{},{}");
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

fn simulate(grid: &mut Grid<Cell>, sand_pos: (usize, usize)) -> u32 {
	let mut sand_count = 0;
	'sim: loop {
		let mut sand_pos = sand_pos;
		if matches!(grid[sand_pos], Cell::Sand) {
			break 'sim;
		}
		loop {
			let check_pos = (sand_pos.0, sand_pos.1 + 1);
			if check_pos.1 >= grid.get_size().1 {
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
			if check_pos.0 >= grid.get_size().0 {
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
	sand_count
}

pub fn solution_1(input: &str) -> String {
	let (mut grid, min) = parse(input, Vec2::new(u32::MAX, 0), Vec2::zero());
	let sand_pos = ((500 - min.x) as usize, (0 - min.y) as usize);
	simulate(&mut grid, sand_pos).to_string()
}

pub fn solution_2(input: &str) -> String {
	let (mut grid, min) = parse(input, Vec2::new(328, 0), Vec2::new(672, 0));
	let sand_pos = ((500 - min.x) as usize, (0 - min.y) as usize);
	grid[sand_pos] = Cell::Sand;
	let mut sand_count = 1;

	let center = sand_pos.0;
	for y in 1..grid.height() {
		let mut last_right = None;
		let mut last_center = None;
		for x in (center - y)..=(center + y) {
			if matches!(grid[(x, y)], Cell::Air) {
				let row = grid.get_row(y - 1).unwrap();
				let left = last_center.unwrap_or(matches!(row[x - 1], Cell::Sand));
				let center = last_right.unwrap_or(matches!(row[x], Cell::Sand));
				let right = matches!(row[x + 1], Cell::Sand);
				last_center = Some(center);
				last_right = Some(right);

				if left || center || right {
					grid[(x, y)] = Cell::Sand;
					sand_count += 1;
				}
			} else {
				last_center = last_right;
				last_right = None;
			}
		}
	}

	let out = grid.to_input(|cell| {
		match cell {
			Cell::Air => '.',
			Cell::Rock => '#',
			Cell::Sand => 'o',
		}
	});

	std::fs::write("dbg.txt", out).unwrap();

	sand_count.to_string()
}
