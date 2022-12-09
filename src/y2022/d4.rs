fn parse<'a>(input: &'a str) -> impl Iterator<Item = ((u32, u32), (u32, u32))> + 'a {
	input.lines().filter_map(|ranges| {
		let (r1, r2) = ranges.split_once(',')?;
		let parse_range = |range: &str| {
			let (a, b) = range.split_once("-")?;
			Some((a.parse::<u32>().ok()?, b.parse::<u32>().ok()?))
		};
		let r1 = parse_range(r1)?;
		let r2 = parse_range(r2)?;
		Some((r1, r2))
	})
}

pub fn solution_1(input: &str) -> String {
	parse(input).filter(|(a, b)| (a.0 <= b.0 && a.1 >= b.1) || (b.0 <= a.0 && b.1 >= a.1)).count().to_string()
}

pub fn solution_2(input: &str) -> String {
	parse(input).filter(|(a, b)| (a.0 >= b.0 && a.0 <= b.1) || (a.1 >= b.0 && a.1 <= b.1) || (b.0 >= a.0 && b.0 <= a.1)).count().to_string()
}