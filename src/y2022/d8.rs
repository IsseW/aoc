use colored::Colorize;
use hashbrown::HashSet;

use crate::helpers::{self, Grid, GridLinearSlice, GridIndex};

pub fn solution_1(input: &str) -> String {
	let grid = Grid::from_input(input, |c| Some((c as u8 - b'0') as i32));
	let mut set = HashSet::new();
	let (width, height) = grid.get_size();
	for (x, col) in grid.columns().enumerate() {
		let mut last = -1;
		for (y, cell) in col.iter().enumerate() {
			if *cell > last {
				set.insert((x, y));
				last = *cell;
			}
		}

		let mut last = -1;
		let mut y = col.len() - 1;
		for cell in col.iter().rev() {
			if *cell > last {
				set.insert((x, y));
				last = *cell;
			}
			y = y.saturating_sub(1);
		}
	}

	for (y, row) in grid.rows().enumerate() {
		let mut last = -1;
		for (x, cell) in row.iter().enumerate() {
			if *cell > last {
				set.insert((x, y));
				last = *cell;
			}
		}

		let mut last = -1;
		let mut x = row.len() - 1;
		for cell in row.iter().rev() {
			if *cell > last {
				set.insert((x, y));
				last = *cell;
			}
			x = x.saturating_sub(1);
		}
	}

	set.len().to_string()
}

pub fn solution_2(input: &str) -> String {
	let grid = Grid::from_input(input, |c| Some((c as u8 - b'0') as u32));
	let (width, height) = grid.get_size();
	grid.enumerate().map(|(v, x, y)| {
		let left = x - (0..x).rev().find(|x| grid.get(*x, y).map_or(false, |vt| vt >= v)).unwrap_or(0);
		let right = (x + 1..width).find(|x| grid.get(*x, y).map_or(false, |vt| vt >= v)).unwrap_or(width - 1) - x;

		let up = y - (0..y).rev().find(|y| grid.get(x, *y).map_or(false, |vt| vt >= v)).unwrap_or(0);
		let down = (y + 1..height).find(|y| grid.get(x, *y).map_or(false, |vt| vt >= v)).unwrap_or(height - 1) - y;

		left * right * up * down
	}).max().unwrap().to_string()
}
