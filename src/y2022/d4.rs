fn parse(input: &str) -> impl Iterator<Item = ((u32, u32), (u32, u32))> + '_ {
	input.lines().map(|ranges| {
		let res: (u32, u32, u32, u32) = strp::scan!(ranges => "{}-{},{}-{}");
		((res.0, res.1), (res.2, res.3))
	})
}

pub fn solution_1(input: &str) -> String {
	parse(input).filter(|(a, b)| (a.0 <= b.0 && a.1 >= b.1) || (b.0 <= a.0 && b.1 >= a.1)).count().to_string()
}

pub fn solution_2(input: &str) -> String {
	parse(input).filter(|(a, b)| (a.0 >= b.0 && a.0 <= b.1) || (a.1 >= b.0 && a.1 <= b.1) || (b.0 >= a.0 && b.0 <= a.1)).count().to_string()
}