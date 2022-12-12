use pathfinding::directed::astar::astar;
use crate::helpers::{self, Grid, GridIndex, GridLinearSlice, OrderedFloat, Cardinals};

fn parse(input: &str) -> (Grid<u8>, (usize, usize), (usize, usize)) {
	let mut i = 0;
	let mut start = 0;
	let mut end = 0;
	let grid = Grid::from_input(input, |c| {
		let res = match c {
			'S' => {
				start = i;
				0
			},
			'E' => {
				end = i;
				b'z' - b'a'
			},
			'a'..='z' => {
				c as u8 - b'a'
			},
			_ => return None,
		};
		i += 1;
		Some(res)
	});
	let (width, height) = grid.get_size();
	let start = (start % width, start / width);
	let end = (end % width, end / width);
	(grid, start, end)
}

pub fn solution_1(input: &str) -> String {
	let (grid, start, end) = parse(input);
	let path = grid.find_path::<Cardinals>(start, end, |a, _, b, _| {
		(*b <= a + 1).then_some(1.0)
	}).unwrap();
	path.1.to_string()
}

pub fn solution_2(input: &str) -> String {
	let (grid, _, end) = parse(input);

	let (_, cost) = grid.enumerate().filter_map(|(v, x, y)| (*v == 0).then_some((x, y))).filter_map(|start| {
		grid.find_path::<Cardinals>(start, end, |a, _, b, _| {
			(*b <= a + 1).then_some(1.0)
		})
	}).min_by_key(|(_, cost)| OrderedFloat(*cost)).unwrap();

	cost.to_string()
}
