use std::str::Lines;

use hashbrown::HashMap;

use crate::helpers;

enum Entry<'a> {
	Directory(HashMap<&'a str, Entry<'a>>),
	File(usize),
}

impl<'a> Entry<'a> {
	fn size(&self) -> usize{
		match self {
			Entry::Directory(files) => files.values().map(|file| file.size()).sum(),
			Entry::File(size) => *size,
		}
	}

	fn walk_entries(&self, f: &mut impl FnMut(&Self)) {
		f(self);
		match self {
			Entry::Directory(dir) => {
				for entry in dir.values() {
					entry.walk_entries(f)
				}
			},
			Entry::File(_) => {},
		}
	}

	fn is_dir(&self) -> bool {
		matches!(self, Entry::Directory(_))
	}
}

fn parse_dir<'a>(lines: &mut Lines<'a>) -> HashMap<&'a str, Entry<'a>> {
	match lines.next().unwrap() {
		"$ ls" => {
			let mut entries = HashMap::new();
			while let Some(line) = lines.next() {
				match line {
					s if line.starts_with("$ cd ") => {
						let name = line.trim_start_matches("$ cd ");
						if name == ".." {
							break;
						}
						let Entry::Directory(dir) = entries.get_mut(name).unwrap() else {
							panic!()
						};
						*dir = parse_dir(lines);
					},
					s if s.starts_with("dir ") => {
						let name = s.trim_start_matches("dir ");
						entries.insert(name, Entry::Directory(HashMap::new()));
					}
					s => {
						let (size, name) = s.split_once(' ').unwrap();
						let size = size.parse().unwrap();

						entries.insert(name, Entry::File(size));
					}
				}
			}
			entries
		},
		s => {
			panic!("unexpected pattern: {s}")
		},
	}
}

fn parse<'a>(input: &'a str) -> Entry<'a> {
	let mut lines = input.lines();
	// Skip root node
	lines.next().unwrap();
	Entry::Directory(parse_dir(&mut lines))
}

pub fn solution_1(input: &str) -> String {
	let mut sum = 0;
	parse(input).walk_entries(&mut |entry| {
		if entry.is_dir() {
			let size = entry.size();
			if size <= 100000 {
				sum += size;
			}
		}
	});
	sum.to_string()
}

pub fn solution_2(input: &str) -> String {
	let root_node = parse(input);
	let total_size = root_node.size();
	let required_size = 30000000 - (70000000 - total_size);
	let mut smallest = total_size;
	root_node.walk_entries(&mut |entry| {
		if entry.is_dir() {
			let size = entry.size();
			if size < smallest && size >= required_size {
				smallest = size;
			}
		}
	});
	smallest.to_string()
}
