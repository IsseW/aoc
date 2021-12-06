use crate::helpers;

pub fn solution_1(input: &str) -> String {
    let mut fish = input
        .split(',')
        .map(|x| x.parse::<u8>().unwrap())
        .collect::<Vec<u8>>();

    for _ in 0..80 {
        for i in 0..fish.len() {
            if fish[i] > 0 {
                fish[i] -= 1;
            } else {
                fish[i] = 6;
                fish.push(8);
            }
        }
    }
    fish.len().to_string()
}

pub fn solution_2(input: &str) -> String {
    let fish = input
        .split(',')
        .map(|x| x.parse::<u8>().unwrap())
        .collect::<Vec<u8>>();
    let mut fish_counts = [0u64; 7];
    let mut fish_add = [0u64; 7];
    for fish in fish {
        fish_counts[fish as usize] += 1;
    }

    for i in 0..256 {
        fish_add[(i + 2) % 7] = fish_counts[i % 7];
        fish_counts[i % 7] += fish_add[i % 7];
        fish_add[i % 7] = 0;
    }
    fish_counts
        .iter()
        .chain(fish_add.iter())
        .sum::<u64>()
        .to_string()
}
