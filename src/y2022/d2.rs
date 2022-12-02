use crate::helpers;

fn parse<'a>(input: &'a str) -> impl Iterator<Item = (u8, u8)> + 'a {
    input.lines().filter_map(|line| {
		let [e, p] = *line.split_whitespace().filter_map(|s| (s.len() == 1).then(|| s.chars().next()).flatten()).collect::<Vec<_>>() else {
			return None;
		};
		Some((e as u8 - b'A', p as u8 - b'X'))
	})
}

pub fn solution_1(input: &str) -> String {
    parse(input)
        .map(|(e, p)| {
            let r = (4 + p - e) % 3;

            let win_points = r * 3;

            let play_points = p + 1;

            (win_points + play_points) as u32
        })
        .sum::<u32>()
        .to_string()
}

pub fn solution_2(input: &str) -> String {
    parse(input)
        .map(|(e, r)| {
            let p = (e + r + 2) % 3;

            let play_points = p + 1;

            let win_points = r * 3;

            (win_points + play_points) as u32
        })
        .sum::<u32>()
        .to_string()
}
