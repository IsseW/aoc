use crate::helpers::Grid;


fn count_slope(grid: &Grid<bool>, slope: (usize, usize)) -> usize {
	let mut i = 0;
	let mut count = 0;
	while slope.1 * i < grid.height() {
		let s = ((slope.0 * i) % grid.width(), slope.1 * i);
		count += grid[s] as usize;
		i += 1;
	}
	count
}

pub fn solution_1(input: &str) -> String {
	let grid = Grid::from_map(input);
	count_slope(&grid, (3, 1)).to_string()
}

pub fn solution_2(input: &str) -> String {
	let grid = Grid::from_map(input);
	const SLOPES: &[(usize, usize)] = &[
		(1, 1),
		(3, 1),
		(5, 1),
		(7, 1),
		(1, 2),
	];

	SLOPES.iter().map(|slope| count_slope(&grid, *slope)).product::<usize>().to_string()
}
