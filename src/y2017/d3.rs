use vek::{num_integer::Roots, Vec2};

use crate::helpers::{self, Grid, spiral::Spiral};
const LEN: usize = 1000;
const ODD_SQUARES: [u32; LEN] = {
	let mut arr = [0; LEN];
	let mut i = 0;
	loop {
		if i >= LEN {
			break;
		}
		arr[i] = i as u32 * 2 + 1;

		i += 1;
	}
	arr
};

pub fn solution_1(input: &str) -> String {
	let cell = input.parse::<i32>().unwrap();
	// let cell: i32 = strp::parse!("{}");
	let layer = ((cell - 1).sqrt() + 1) / 2;
	println!("layer: {layer}");

	let corner = (layer * 2 + 1).pow(2);
	let mut pos = Vec2::broadcast(layer);
	let mut left = corner - cell;

	let layer_len = layer * 2;
	let mut do_dir = |dir: Vec2<i32>| {
		if left > 0 {
			left -= layer_len;
			println!("pos: {pos}");
			pos += dir * (layer_len + left.min(0));
		}
	};
	do_dir(-Vec2::unit_x());
	do_dir(-Vec2::unit_y());
	do_dir(Vec2::unit_x());
	do_dir(Vec2::unit_y());

	println!("pos: {pos}");
	
	(pos.x.abs() + pos.y.abs()).to_string()
}

pub fn solution_2(input: &str) -> String {
	let input = input.parse::<u32>().unwrap();
	let mut grid = Grid::new(11, 11);
	let center = (grid.width() / 2, grid.height() / 2);

	let index = |v: Vec2<i32>| {
		(center.0.checked_add_signed(v.x as isize).unwrap(), center.1.checked_add_signed(v.y as isize).unwrap())
	};
	let neighbors = [
		Vec2::new(1, 0),
		Vec2::new(1, 1),
		Vec2::new(0, 1),
		Vec2::new(-1, 1),
		Vec2::new(-1, 0),
		Vec2::new(-1, -1),
		Vec2::new(0, -1),
		Vec2::new(1, -1),
	];
	// grid[center] = 1u32;
	for (i, v) in Spiral::new().enumerate().skip(1).take(10) {
		let val: u32 = neighbors.iter().map(|o| v + o).map(index).map(|i| grid[i]).sum();
		if val > input {
			return val.to_string();
		}
		grid[index(v)] = i as u32;
	}
	println!("{grid}");

	"Not answer found".into()
}

// 37 36 35 34 33 32 31
// 38 17 16 15 14 13 30
// 39 18  5  4  3 12 29
// 40 19  6  1  2 11 28
// 41 20  7  8  9 10 27
// 42 21 22 23 24 25 26
// 43 44 45 46 47 48 49