use hashbrown::HashSet;
use vek::Vec2;

use crate::helpers;

pub fn solution_1(input: &str) -> String {
	let mut set = HashSet::new();
	let mut head_pos = Vec2::<i32>::zero();
	let mut tail_pos = Vec2::<i32>::zero();
	set.insert(tail_pos);
	input.lines().for_each(|line| {
		let (dir, amount) = line.split_once(' ').unwrap();
		let dir = match dir {
			"R" => Vec2::new(1, 0),
			"L" => Vec2::new(-1, 0),
			"U" => Vec2::new(0, -1),
			"D" => Vec2::new(0, 1),
			_ => panic!(),
		};
		let amount = amount.parse::<i32>().unwrap();
		for _ in 0..amount {
			head_pos += dir;
			let diff = (tail_pos - head_pos).as_::<f32>().normalized().map(|e| e.round() as i32);
			tail_pos = head_pos + diff;
			set.insert(tail_pos);
		}
	});

	set.len().to_string()
}

pub fn solution_2(input: &str) -> String {
	let mut set = HashSet::new();
	let mut positions = [Vec2::<i32>::zero(); 10];
	let mut min = Vec2::<i32>::zero();
	let mut max = Vec2::<i32>::zero();
	set.insert(positions[8]);
	input.lines().for_each(|line| {
		let (dir, amount) = line.split_once(' ').unwrap();
		let dir = match dir {
			"R" => Vec2::new(1, 0),
			"L" => Vec2::new(-1, 0),
			"U" => Vec2::new(0, -1),
			"D" => Vec2::new(0, 1),
			_ => panic!(),
		};
		let amount = amount.parse::<i32>().unwrap();
		for _ in 0..amount {
			positions[0] += dir;
			let mut head_pos = positions[0];
			for tail_pos in &mut positions[1..] {
				let diff = *tail_pos - head_pos;
				if diff.magnitude_squared() == 0 {
					continue;
				}
				let diff = diff.as_::<f32>().normalized().map(|e| e.round() as i32);
				*tail_pos = head_pos + diff;
				head_pos = *tail_pos;
			}
			set.insert(*positions.last().unwrap());
			min = min.map2(positions.iter().copied().reduce(|a, b| a.map2(b, |a, b| a.min(b))).unwrap(), |a, b| a.min(b));
			max = max.map2(positions.iter().copied().reduce(|a, b| a.map2(b, |a, b| a.max(b))).unwrap(), |a, b| a.max(b));
		}
	});
	set.len().to_string()
}
