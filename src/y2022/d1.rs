
fn elf_foods(input: &str) -> impl Iterator<Item = u64> + '_ {
	input.split("\n\n").map(|elf| {
		elf.lines().filter_map(|line| line.parse::<u64>().ok()).sum::<u64>()
	})
}

pub fn solution_1(input: &str) -> String {
	elf_foods(input).max().unwrap().to_string()
}

pub fn solution_2(input: &str) -> String {
	let mut elfs = elf_foods(input).collect::<Vec<_>>();
	elfs.sort();
	let max: u64 = elfs[elfs.len() - 3..].iter().sum();
	max.to_string()
}
