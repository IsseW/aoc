use itertools::Itertools;
use vek::Vec2;

pub fn solve(input: &str, mul: u64) -> String {
    let mut galaxies = Vec::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.bytes().enumerate() {
            if c == b'#' {
                galaxies.push(Vec2::new(x, y).as_::<u64>());
            }
        }
    }

    for e in 0..2 {
        galaxies.sort_unstable_by_key(|v| v[e]);
        let mut last = galaxies[0][e];
        let mut shift = 0;
        for i in 1..galaxies.len() {
            shift += (galaxies[i][e] - last).saturating_sub(1) * (mul - 1);
            last = galaxies[i][e];
            galaxies[i][e] += shift;
        }
    }

    galaxies
        .iter()
        .tuple_combinations::<(_, _)>()
        .map(|(a, b)| a.map2(*b, |a, b| a.abs_diff(b)).sum())
        .sum::<u64>()
        .to_string()
}

pub fn solution_1(input: &str) -> String {
    solve(input, 2)
}

pub fn solution_2(input: &str) -> String {
    solve(input, 1000000)
}
