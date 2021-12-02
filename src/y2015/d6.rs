use itertools::Itertools;

use crate::helpers::*;

enum SwitchMode {
    Turn(bool),
    Toggle,
}

impl SwitchMode {
    fn switch(&self, toggle: &mut bool) {
        match self {
            SwitchMode::Turn(b) => *toggle = *b,
            SwitchMode::Toggle => *toggle = !*toggle,
        }
    }

    fn increase(&self, brightness: &mut usize) {
        match self {
            SwitchMode::Turn(increase) => {
                if *increase {
                    *brightness += 1
                } else if *brightness > 0 {
                    *brightness -= 1
                }
            }
            SwitchMode::Toggle => *brightness += 2,
        }
    }
}

fn parse_coords(string: &str) -> (usize, usize) {
    let split = string.split_once(',').unwrap();
    (
        usize::from_str_radix(split.0, 10).unwrap(),
        usize::from_str_radix(split.1, 10).unwrap(),
    )
}

pub fn solution_1(input: &str) -> String {
    let mut grid = Grid::<bool>::new(1000, 1000);
    input.lines().for_each(|line| {
        if line.is_empty() {
            return;
        }
        let mut split = line.split_whitespace();

        let mode = match split.next() {
            Some("turn") => SwitchMode::Turn(match split.next() {
                Some("on") => true,
                _ => false,
            }),
            _ => SwitchMode::Toggle,
        };
        let start = parse_coords(split.next().unwrap());
        split.next();
        let end = parse_coords(split.next().unwrap());

        for y in start.1..=end.1 {
            for x in start.0..=end.0 {
                mode.switch(&mut grid[(x, y)]);
            }
        }
    });
    grid.iter().counts_by(|c| c)[&true].to_string()
}

pub fn solution_2(input: &str) -> String {
    let mut grid = Grid::<usize>::new(1000, 1000);
    input.lines().for_each(|line| {
        if line.is_empty() {
            return;
        }
        let mut split = line.split_whitespace();

        let mode = match split.next() {
            Some("turn") => SwitchMode::Turn(match split.next() {
                Some("on") => true,
                _ => false,
            }),
            _ => SwitchMode::Toggle,
        };
        let start = parse_coords(split.next().unwrap());
        split.next();
        let end = parse_coords(split.next().unwrap());

        for y in start.1..=end.1 {
            for x in start.0..=end.0 {
                mode.increase(&mut grid[(x, y)]);
            }
        }
    });
    grid.iter().sum::<usize>().to_string()
}
