use itertools::Itertools;

pub fn solution_1(input: &str) -> String {
	let len = input.len();
	input.chars().cycle().take(len + 1).map(|c| c.to_digit(10).unwrap()).tuple_windows::<(u32, u32)>().filter_map(|(a, b)| (a == b).then_some(a)).sum::<u32>().to_string()
}

pub fn solution_2(input: &str) -> String {
	let v = input.chars().map(|c| c.to_digit(10).unwrap()).collect_vec();
	let mut sum = 0;
	for i in 0..v.len() {
		if v[i] == v[(i + v.len() / 2) % v.len()] {
			sum += v[i];
		}
	}
	sum.to_string()
}
