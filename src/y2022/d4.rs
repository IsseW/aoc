fn parse<'a>(input: &'a str) -> impl Iterator<Item = ((u32, u32), (u32, u32))> + 'a {
	input.lines().filter_map(|ranges| {
		let res: (u32, u32, u32, u32) = strp::try_scan!(ranges => "{}-{},{}-{}").ok()?;
		Some(((res.0, res.1), (res.2, res.3)))
	})
}

pub fn solution_1(input: &str) -> String {
	parse(input).filter(|(a, b)| (a.0 <= b.0 && a.1 >= b.1) || (b.0 <= a.0 && b.1 >= a.1)).count().to_string()
}

pub fn solution_2(input: &str) -> String {
	parse(input).filter(|(a, b)| (a.0 >= b.0 && a.0 <= b.1) || (a.1 >= b.0 && a.1 <= b.1) || (b.0 >= a.0 && b.0 <= a.1)).count().to_string()
}