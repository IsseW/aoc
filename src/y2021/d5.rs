use crate::helpers::*;

type Line = ((i32, i32), (i32, i32));

fn parse_lines(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|l| {
            let (a, b) = l.split_once(" -> ").unwrap();
            let (a, b) = (a.split_once(',').unwrap(), b.split_once(',').unwrap());
            let (a, b) = (
                (a.0.parse::<i32>().unwrap(), a.1.parse::<i32>().unwrap()),
                (b.0.parse::<i32>().unwrap(), b.1.parse::<i32>().unwrap()),
            );
            ((a.0, a.1), (b.0, b.1))
        })
        .collect::<Vec<_>>()
}

fn find_grid_size(lines: &[Line]) -> usize {
    (lines
        .iter()
        .map(|l| l.0 .0.max(l.1 .1).max(l.1 .0.max(l.1 .1)))
        .max()
        .unwrap()
        + 1) as usize
}

pub fn solution_1(input: &str) -> String {
    let lines = parse_lines(input);

    let grid_size = find_grid_size(&lines);
    let mut grid = Grid::<u8>::new(grid_size, grid_size);

    for &l in lines.iter().filter(|(a, b)| a.0 == b.0 || a.1 == b.1) {
        LineIter::from(l).for_each(|(x, y)| grid[(x as usize, y as usize)] += 1);
    }
    grid.iter().filter(|&&v| v > 1).count().to_string()
}

pub fn solution_2(input: &str) -> String {
    let lines = parse_lines(input);
    let grid_size = find_grid_size(&lines);
    let mut grid = Grid::<u8>::new(grid_size, grid_size);

    for l in lines {
        LineIter::from(l).for_each(|(x, y)| grid[(x as usize, y as usize)] += 1);
    }

    grid.iter().filter(|&&v| v > 1).count().to_string()
}
