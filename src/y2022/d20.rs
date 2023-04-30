use std::collections::VecDeque;


fn parse(input: &str) -> impl Iterator<Item = i64> + '_ {
	input.lines().filter_map(|num| num.parse().ok())
}

fn mix(numbers: impl Iterator<Item = i64>, times: u32) -> i64 {
	let mut numbers = numbers.enumerate().collect::<VecDeque<_>>();

	for _ in 0..times {
		for i in 0..numbers.len() {
			let index = numbers.iter().position(|(index, _)| *index == i).unwrap();
			numbers.rotate_left(index);
			let item = numbers.pop_front().unwrap();
			numbers.rotate_left(item.1.rem_euclid(numbers.len() as i64) as usize);
			numbers.push_front(item);
		}
	}
	
	numbers.into_iter().map(|(_, n)| n).cycle().skip_while(|n| *n != 0).step_by(1000).skip(1).take(3).sum::<i64>()
}

pub fn solution_1(input: &str) -> String {
	mix(parse(input), 1).to_string()
}

pub fn solution_2(input: &str) -> String {
	mix(parse(input).map(|n| n * 811589153), 10).to_string()
}
