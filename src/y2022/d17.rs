use std::cmp::Ordering;
use itertools::Itertools;

#[derive(Clone, Copy, Debug)]
enum Dir {
	Left,
	Right,
}

fn parse(input: &str) -> impl ExactSizeIterator<Item = Dir> + '_ + Clone {
	input.bytes().map(|c| {
		match c as char {
			'<' => Dir::Left,
			'>' => Dir::Right,
			c => panic!("Unexpected char {c:?}"),
		}
	})
}

fn sim_shape(i: usize, jets: &mut impl Iterator<Item = Dir>, grid: &mut BitGrid) -> (usize, usize) {
	let mut shape = SHAPES[i % SHAPES.len()];
	let mut height = grid.highest;

	let mut jet = || unsafe { jets.next().unwrap_unchecked() };
	shape.shift(jet());
	shape.shift(jet());
	shape.shift(jet());
	loop {
		let mut new_shape = shape;
		
		if new_shape.shift(jet()) && !grid.collides_with(new_shape, height) {
			shape = new_shape;
		}
		if let Some(next_height) = height.checked_sub(1) {
			if grid.collides_with(shape, next_height) {
				break;
			} else {
				height = next_height;
			}
		} else {
			break;
		}
	}
	grid.highest = grid.highest.max(height.wrapping_add(shape.height as usize));
	grid.add_shape(shape, height);
	(shape.min_x as usize, height)
}
const WIDTH: usize = 7;
#[derive(Clone, Copy)]
struct Shape {
	data: ShapeStorage,
	min_x: u8,
	max_x: u8,
	height: u8
}

const SHAPES: [Shape; 5] = [
	Shape {
		data: 0b11110,
		min_x: 2,
		max_x: 6,
		height: 1,
	},
	Shape {
		data: 0b0001000_0011100_0001000,
		min_x: 2,
		max_x: 5,
		height: 3,
	},
	Shape {
		data: 0b0000100_0000100_0011100,
		min_x: 2,
		max_x: 5,
		height: 3,
	},
	Shape {
		data: 0b0010000_0010000_0010000_0010000,
		min_x: 2,
		max_x: 3,
		height: 4,
	},
	Shape {
		data: 0b0011000_0011000,
		min_x: 2,
		max_x: 4,
		height: 2,
	},
];

impl Shape {
	fn shift(&mut self, dir: Dir) -> bool {
		match dir {
			Dir::Left if self.min_x > 0 => {
				self.min_x -= 1;
				self.max_x -= 1;
				self.data <<= 1;
				true
			},
			Dir::Right if self.max_x < WIDTH as u8 =>  {
				self.min_x += 1;
				self.max_x += 1;
				self.data >>= 1;
				true
			},
			_ => {
				false
			}
		}
	}
}

type Storage = u64;
type ShapeStorage = u32;

#[derive(Clone)]
struct BitGrid<const W: usize = WIDTH> {
	rows: Vec<Storage>,
	height: usize,
	highest: usize,
}

impl<const W: usize> BitGrid<W> {
	const BITS: usize = (Storage::BITS as usize / W) * W;
	const CELL_HEIGHT: usize = Self::BITS / W;
	const SHAPE_BITS: usize = (ShapeStorage::BITS as usize / W) * W;
	const SHAPE_HEIGHT: usize = Self::SHAPE_BITS / W;
	fn new(height: usize) -> Self {
		Self {
			height,
			highest: 0,
			rows: vec![0; height / Self::CELL_HEIGHT],
		}
	}

	fn collides_with(&self, shape: Shape, height: usize) -> bool {
		if height > self.highest {
			return false;
		}
		let cell = height / Self::CELL_HEIGHT;
		let o_cell = height.saturating_add(shape.height as usize) / Self::CELL_HEIGHT;

		let p = height % Self::CELL_HEIGHT;
		let shape1 = (shape.data as Storage).wrapping_shl((p * W) as u32);

		// NOTE: p is always less than CELL_HEIGHT
		let p = Self::CELL_HEIGHT - p;
		let shape2 = (shape.data as Storage).wrapping_shr((p * W) as u32);
		o_cell >= self.rows.len() || (self.rows[cell] & shape1) != 0
		|| (o_cell != cell && (self.rows[o_cell] & shape2) != 0)
	}

	fn add_shape(&mut self, shape: Shape, height: usize) {
		let cell = height / Self::CELL_HEIGHT;
		let o_cell = height.saturating_add(shape.height as usize) / Self::CELL_HEIGHT;

		let p = height % Self::CELL_HEIGHT;
		let shape1 = (shape.data as Storage).wrapping_shl((p * W) as u32);

		self.rows[cell] |= shape1;
		self.rows[cell] &= !(Storage::MAX << Self::BITS);

		if o_cell != cell {
			let p = Self::CELL_HEIGHT - p;
			let shape2 = (shape.data as Storage).wrapping_shr((p * W) as u32);

			self.rows[o_cell] |= shape2;
		}
	}

	fn get(&self, row: usize, col: usize) -> bool {
		let cell = row / Self::CELL_HEIGHT;
		let r = row % Self::CELL_HEIGHT;

		((self.rows[cell] >> (r * W + col)) & 1) != 0
	}

	fn print(&self) {
		for row in (0..self.highest + 1).rev() {
			print!("|");
			for col in (0..W).rev() {
				if self.get(row, col) {
					print!("#");
				} else {
					print!(".");
				}
			}
			println!("|");
		}

		println!("{}", "-".repeat(2 + W));
	}
}

fn simulate(jets: impl Iterator<Item = Dir> + Clone, num: usize) -> usize {
	let mut jets = jets.cycle();
	let height = SHAPES.iter().map(|shape| shape.height as usize).sum::<usize>() * num;
	let mut grid = BitGrid::new(height);
	for i in 0..num {
		sim_shape(i, &mut jets, &mut grid);
	}
	grid.highest
}

pub fn solution_1(input: &str) -> String {
	simulate(parse(input).collect_vec().into_iter(), 2022).to_string()
}

struct Cycle {
	xs: Vec<usize>,
	k_start: usize,
	start_height: usize,
}

pub fn solution_2(input: &str) -> String {
	let n = 1000000000000;
	let input = parse(input).collect_vec();

	let input_cycle_len = input.len() * SHAPES.len();
	
	let mut jets = input.iter().copied().cycle();

	let height = SHAPES.iter().map(|shape| shape.height as usize).sum::<usize>() * input_cycle_len * 100;
	let mut grid = BitGrid::new(height);
	let mut maps = Vec::<Cycle>::new();
	let mut heights = Vec::<usize>::new();

	let (cycle_height, cycle_start, cycle_end) = (0..).find_map(|k| {
		let last_highest = grid.highest;
		heights.reserve(input_cycle_len);
		let x = (0..input_cycle_len).map(|i| {
			let i = i + k * input_cycle_len;
			let res = sim_shape(i, &mut jets, &mut grid);
			heights.push(grid.highest);
			res.0
		}).collect_vec();

		let search_start = std::time::Instant::now();
		let mut i = 0;
		let res = match maps.binary_search_by(|cycle| {
			while i < input_cycle_len && cycle.xs[i] == x[i] {
				i += 1;
			}

			if i >= input_cycle_len {
				Ordering::Equal
			} else {
				x[i].cmp(&cycle.xs[i])
			}
		}) {
			Ok(i) => {
				
				Some((last_highest - maps[i].start_height, maps[i].k_start * input_cycle_len, k * input_cycle_len))
			},
			Err(i) => {
				maps.insert(i, Cycle {
					xs: x,
					k_start: k,
					start_height: last_highest,
				});
				None
			}
		};
	
		res
	}).unwrap();

	let cycle_len = cycle_end - cycle_start;
	let rest = (n - cycle_start) % cycle_len;
	let rest = heights[cycle_start + rest - 1];

	(cycle_height * (n / cycle_len) + rest).to_string()
}
