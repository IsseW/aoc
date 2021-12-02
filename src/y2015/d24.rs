use itertools::Itertools;

fn parse(input: &str) -> (Vec<u64>, u64) {
    let weights = input
        .lines()
        .map(|line| u64::from_str_radix(line, 10).unwrap())
        .collect_vec();
    let total = weights.iter().sum::<u64>();
    (weights, total)
}

fn optimal_split(weights: Vec<u64>, target_weight: u64) -> u64 {
    (1..(weights.len()))
        .find_map(|i| {
            weights
                .iter()
                .combinations(i)
                .filter(|group| group.iter().map(|&&v| v).sum::<u64>() == target_weight)
                .map(|group| group.iter().map(|&&v| v).product::<u64>())
                .min()
        })
        .unwrap()
}

pub fn solution_1(input: &str) -> String {
    let (weights, total) = parse(input);
    let target_weight = total / 3;
    optimal_split(weights, target_weight).to_string()
}

pub fn solution_2(input: &str) -> String {
    let (weights, total) = parse(input);
    let target_weight = total / 4;
    optimal_split(weights, target_weight).to_string()
}
