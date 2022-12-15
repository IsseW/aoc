use itertools::Itertools;
use vek::{Aabr, Vec2};

use crate::helpers::{self, Grid};

struct Sensor {
	pos: (i64, i64),
	beacon: (i64, i64),
}

impl Sensor {
	fn dist(&self) -> i64 {
		manhattan(self.pos, self.beacon)
	}

	fn cannot_contain_beacon(&self, pos: (i64, i64)) -> bool {
		pos != self.beacon && self.contains(pos)
	}
	fn contains(&self, pos: (i64, i64)) -> bool {
		manhattan(self.pos, pos) <= self.dist()
	}
}

fn manhattan(a: (i64, i64), b: (i64, i64)) -> i64 {
	(a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn parse(input: &str) -> impl Iterator<Item = Sensor> + '_ {
	input.lines().map(|line| {
		let (x, y, bx, by) = strp::scan!(line => "Sensor at x={}, y={}: closest beacon is at x={}, y={}");
		Sensor { pos: (x, y), beacon: (bx, by) }
	})
}

pub fn solution_1(input: &str) -> String {
	let sensors = parse(input).collect_vec();
	let min = sensors.iter().map(|sensor| sensor.pos.0 - sensor.dist()).min().unwrap();
	let max = sensors.iter().map(|sensor| sensor.pos.0 + sensor.dist()).max().unwrap();
	let y = 2000000;
	let mut count = 0;
	for x in min..=max {
		if sensors.iter().any(|sensor| sensor.cannot_contain_beacon((x, y))) {
			count += 1;
		}
	}

	count.to_string()
}

pub fn solution_2(input: &str) -> String {
	const RANGE: i64 = 4000000;
	let sensors = parse(input).collect_vec();
	let mut y = 0;
	loop {
		let mut x = 0;
		loop {
			if let Some(sensor) = sensors.iter().find(|s| s.contains((x, y))) {
				let dist = sensor.dist();
				let w = dist - (sensor.pos.1 - y).abs();
				x = sensor.pos.0 + w + 1;
				if x >= RANGE {
					x = 0;
					y += 1;
				}
			} else {
				return (x * RANGE + y).to_string()
			}
		}
	}
}
