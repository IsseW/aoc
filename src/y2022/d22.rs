use std::ops::RangeInclusive;

use enum_map::EnumMap;
use itertools::Itertools;

use crate::helpers::{Grid, Dir, GridLinearSlice, GridIndex, GridIndexMut, store::{Id, Store}};

#[derive(Debug)]
enum Instruction {
	Noop,
	Clockwise,
	CounterClockwise,
	Move(u32),
}

fn instructions(input: &str) -> impl Iterator<Item = Instruction> + '_ {
	let mut num_start = None;
	input.char_indices().chain(std::iter::once((input.len(), '\0'))).filter_map(move |(i, c)| {
		let instr = match c {
			'R' => Instruction::Clockwise,
			'L' => Instruction::CounterClockwise,
			'0'..='9' => {
				if num_start.is_none() {
					num_start = Some(i);
				}
				return None;
			},
			_ => Instruction::Noop,
		};

		let res = num_start.take().map(|num_start| { 
			let n = input[num_start..i].parse().unwrap();

			Instruction::Move(n)
		});

		Some(res.into_iter().chain(std::iter::once(instr)))
	})
	.flatten()
}

trait ChunkType {
	type Ref<T>;
}

struct Temp;

impl ChunkType for Temp {
    type Ref<T> = Option<Id<T>>;
}

struct Finished;

impl ChunkType for Finished {
    type Ref<T> = (Dir, Id<T>);
}

struct Chunk<T: ChunkType> {
	connected: EnumMap<Dir, T::Ref<Self>>,
	tiles: Grid<bool>,
}

struct Chunks {
	chunks: Store<Chunk<Finished>>,
}
/*
impl Chunks {
	fn from_input(input: &str) -> Self {
		let full_grid = Grid::<Option<bool>>::from_map(input);
		let mut chunk_grid = Grid::new(full_grid.width(), full_grid.height());
		let mut chunks = Store::<Chunk<Temp>>::new();

		for (tile, x, y) in full_grid.enumerate() {
			if tile.is_none() || chunk_grid[(x, y)].is_some() {
				continue;
			}
			let id = chunks.construct(|id| {
				let mut width = 0;
				let edges = EnumMap::<Dir, Option<Option<Id<Chunk<Temp>>>>>::default();
				for x in x..full_grid.width() {
					if full_grid[(x, y)]
					let edge = full_grid[(x, y)].map(|_| chunk_grid[(x, y)]);
					width += 1;
					chunk_grid[(x, y)] = Some(id);
				}

				todo!()
			});
		}
		
		todo!()
	}
}
*/

pub fn solution_1(input: &str) -> String {
	let (map, input) = input.split_once("\n\n").unwrap();
	let grid = Grid::<Option<bool>>::from_map(map);
	let mut text_grid = grid.mapped(|t| match t {
		Some(true) => '#',
		Some(false) => '.',
		None => ' ',
	});

	println!("{}", grid.to_map());

	fn range_linear_slice<'a, S: GridLinearSlice<'a, Output = Option<bool>> + 'a + Copy>(slice: S) -> RangeInclusive<isize> {
		let start = slice.iter().position(|p| {
			p.is_some()
		}).unwrap();
		let end = slice.len() - slice.iter().rev().position(|p| {
			p.is_some()
		}).unwrap();

		start as isize..=end as isize
	}

	let row_ranges = grid.rows().map(range_linear_slice).collect_vec();
	let column_ranges = grid.columns().map(range_linear_slice).collect_vec();
	let x = *row_ranges[0].start();
	let (mut x, mut y) = (x + grid.get_row(0).unwrap().iter().skip(x as usize).position(|t| t.map_or(false, |t| !t)).unwrap() as isize, 0);

	let mut dir = Dir::Right;

	for instr in instructions(input) {
		match instr {
			Instruction::Noop => {},
			Instruction::Clockwise => dir = dir.clockwise(),
			Instruction::CounterClockwise => dir = dir.cc_clockwise(),
			Instruction::Move(n) => {
				match dir {
					Dir::Right => {
						for _ in 0..n {
							*text_grid.get_mut(x, y).unwrap() = '>';
							let mut n_x = x + 1;
							if grid.get(n_x, y).copied().flatten().is_none() {
								n_x = *row_ranges[y as usize].start();
							}
							match grid.get(n_x, y).copied().flatten() {
								Some(true) => {
									break;
								},
								Some(false) => {},
								None => panic!(),
							}
							x = n_x;
						}
					},
					Dir::Up => {
						for _ in 0..n {
							*text_grid.get_mut(x, y).unwrap() = '^';
							let mut n_y = y - 1;
							if grid.get(x, n_y).copied().flatten().is_none() {
								n_y = *column_ranges[x as usize].start();
							}
							match grid.get(x, n_y).copied().flatten() {
								Some(true) => {
									break;
								},
								Some(false) => {},
								None => panic!(),
							}
							y = n_y;
						}
					},
					Dir::Left => {
						for _ in 0..n {
							*text_grid.get_mut(x, y).unwrap() = '<';
							let mut n_x = x - 1;
							if grid.get(n_x, y).copied().flatten().is_none() {
								n_x = *row_ranges[y as usize].start();
							}
							match grid.get(n_x, y).copied().flatten() {
								Some(true) => {
									break;
								},
								Some(false) => {},
								None => panic!(),
							}
							x = n_x;
						}
					},
					Dir::Down => {
						for _ in 0..n {
							*text_grid.get_mut(x, y).unwrap() = 'v';
							let mut n_y = y + 1;
							if grid.get(x, n_y).copied().flatten().is_none() {
								n_y = *column_ranges[x as usize].start();
							}
							match grid.get(x, n_y).copied().flatten() {
								Some(true) => {
									break;
								},
								Some(false) => {},
								None => panic!(),
							}
							y = n_y;
						}
					},
				}
			},
		}
		match dir {
			Dir::Right => {
				*text_grid.get_mut(x, y).unwrap() = 'x';
			},
			Dir::Up => {
				*text_grid.get_mut(x, y).unwrap() = 'x';
			},
			Dir::Left => {
				*text_grid.get_mut(x, y).unwrap() = 'x';
			},
			Dir::Down => {
				*text_grid.get_mut(x, y).unwrap() = 'x';
			}
		}
	}

	println!("{}", text_grid.to_map());
	let f = match dir {
		Dir::Right => 0,
		Dir::Up => 3,
		Dir::Left => 2,
		Dir::Down => 1,
	};
	println!("row: {y}, column: {x}, facing: {f}");

	let password = (y + 1) * 1000 + (x + 1) * 4 + f;

	password.to_string()
}

pub fn solution_2(input: &str) -> String {
	"Not yet implemented".into()
}
