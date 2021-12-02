use itertools::Itertools;

pub fn solution_1(input: &str) -> String {
    input
        .split_whitespace()
        .into_iter()
        .map(|present| {
            let split: (u32, u32, u32) = present
                .split('x')
                .map(|side| u32::from_str_radix(side, 10).unwrap())
                .collect_tuple()
                .unwrap();
            let sides: [u32; 3] = [split.0 * split.1, split.0 * split.2, split.1 * split.2];
            sides.iter().map(|s| s * 2).sum::<u32>() + sides.iter().min().unwrap()
        })
        .sum::<u32>()
        .to_string()
}

pub fn solution_2(input: &str) -> String {
    input
        .split_whitespace()
        .into_iter()
        .map(|present| {
            let mut iter = present
                .split('x')
                .map(|side| u32::from_str_radix(side, 10).unwrap());
            let split = [
                iter.next().unwrap(),
                iter.next().unwrap(),
                iter.next().unwrap(),
            ];

            let biggest = split.iter().max().unwrap();

            let mut iter = split
                .iter()
                .filter(|&t| t as *const u32 != biggest as *const u32);

            2 * iter.next().unwrap() + 2 * iter.next().unwrap() + split[0] * split[1] * split[2]
        })
        .sum::<u32>()
        .to_string()
}
