use rayon::iter::{ParallelBridge, ParallelIterator};


fn get_crabs(input: &str) -> Vec<u32> {
    let mut crabs = input
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    crabs.sort();
    crabs
}

pub fn solution_1(input: &str) -> String {
    let crabs = get_crabs(input);
    let average = crabs[crabs.len() / 2];
    crabs
        .iter()
        .map(|&x| (x.max(average) - x.min(average)))
        .sum::<u32>()
        .to_string()
}

pub fn solution_2(input: &str) -> String {
    let crabs = get_crabs(input);
    (0..=crabs[crabs.len() - 1])
        .par_bridge()
        .map(|average| {
            crabs
                .iter()
                .map(|&crab| {
                    let distance = crab.max(average) - crab.min(average);
                    (distance + 1) * distance / 2
                })
                .sum::<u32>()
        })
        .min()
        .unwrap()
        .to_string()
}
