use num_traits::SaturatingSub;
use rayon::prelude::{ParallelIterator, ParallelDrainRange, IntoParallelRefIterator, IndexedParallelIterator, ParallelBridge};
use vek::Vec4;


// x = ore,
// y = clay,
// z = obsidian,
// w = geode,
#[derive(Default, Clone)]
struct Factory {
	rates: Vec4<u32>,
	amounts: Vec4<u32>,

	minutes_left: u32,
}

impl Factory {
	fn finish(mut self) -> Self {
		self.amounts += self.rates * self.minutes_left;
		self.minutes_left = 0;
		self
	}
}

#[derive(Default)]
struct Blueprint {
	costs: Vec4<Vec4<u32>>,
}

fn get_robot<'a, T>(v: &'a Vec4<T>, s: &str) -> &'a T {
	match s {
		"ore" => &v.x,
		"clay" => &v.y,
		"obsidian" => &v.z,
		"geode" => &v.w,
		_ => unreachable!()
	}
}

fn get_robot_mut<'a, T>(v: &'a mut Vec4<T>, s: &str) -> &'a mut T {
	match s {
		"ore" => &mut v.x,
		"clay" => &mut v.y,
		"obsidian" => &mut v.z,
		"geode" => &mut v.w,
		_ => panic!("Unexpected key: {s}"),
	}
}

fn parse(input: &str) -> impl Iterator<Item = Blueprint> + '_ {
	input.lines().map(|line| {
		let (_, rest) = line.split_once(": ").unwrap();
		let mut blueprint = Blueprint::default();

		for robot in rest.split(". ") {
			let mut w = robot.split_whitespace();
			w.next();
			let robot = w.next().unwrap();
			w.next();
			w.next();
			let mut cost  = Vec4::zero();
			while let Some(c) = w.next() {
				let ty = w.next().unwrap().trim_end_matches('.');
				*get_robot_mut(&mut cost, ty) = c.parse().unwrap();
				w.next();
			}
			*get_robot_mut(&mut blueprint.costs, robot) = cost;
		}

		blueprint
	})
}

fn compare_aqc(a: Vec4<Option<u32>>, b: Vec4<Option<u32>>) -> bool {
	a.iter().zip(b.iter()).rev().find_map(|(a, b)| {
		match (a, b) {
			(Some(a), Some(b)) => Some(a >= b),
			(Some(_), None) => Some(true),
			(None, Some(_)) => Some(false),
			(None, None) => None,
		}
	}).unwrap()
}

fn solve(factory: Factory, blueprint: &Blueprint) -> Factory {
	let mut factories = vec![factory.clone()];
	let mut best_factory = factory;
	let maxs = blueprint.costs.reduce(|a, b| a.map2(b, |a, b| a.max(b))).with_w(u32::MAX);
	while !factories.is_empty() {
		for factory in &factories {
			let factory = factory.clone().finish();
			if best_factory.amounts.w < factory.amounts.w {
				best_factory = factory;
			}
		}
		let ext = factories.par_drain(..).flat_map(|factory| {
			blueprint.costs.par_iter().enumerate().filter_map(move |(i, cost)| {
				if factory.rates[i] >= maxs[i] {
					return None;
				}
				let turns = cost.saturating_sub(&factory.amounts).zip(factory.rates).into_array().try_map(|(a, b)| if a == 0 {
					Some(0)
				} else if b == 0 {
					None
				} else {
					Some((a + b - 1) / b)
				})?.into_iter().max().unwrap();
				
				if turns + 1 + (i != 3) as u32 * 3 >= factory.minutes_left {
					return None;
				}
				Some((turns, cost, i))
			}).map(move |(turns, cost, i)| {
				let mut factory = factory.clone();
				factory.minutes_left -= turns;
				factory.amounts += factory.rates * turns;
				factory.amounts -= *cost;

				factory.minutes_left -= 1;
				factory.amounts += factory.rates;

				factory.rates[i] += 1;
				factory
			})
		}).collect::<Vec<_>>();

		factories.extend(ext);
	}
	best_factory
}

pub fn solution_1(input: &str) -> String {
	let def_factory = Factory {
		rates: Vec4::unit_x(),
		amounts: Vec4::zero(),
		minutes_left: 24,
	};
	parse(input).enumerate().map(|(i, blueprint)| {
		let id = (i + 1) as u32;
		let factory = solve(def_factory.clone(), &blueprint);
		factory.amounts.w * id
	}).sum::<u32>().to_string()
}

pub fn solution_2(input: &str) -> String {
	let def_factory = Factory {
		rates: Vec4::unit_x(),
		amounts: Vec4::zero(),
		minutes_left: 32,
	};
	parse(input).take(3).par_bridge().map(|blueprint| {
		let factory = solve(def_factory.clone(), &blueprint);
		factory.amounts.w
	}).product::<u32>().to_string()
}
