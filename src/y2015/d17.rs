use itertools::Itertools;

fn parse(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut containers: Vec<u32> = input
        .lines()
        .map(|line| u32::from_str_radix(line.trim(), 10).unwrap())
        .collect();
    containers.sort_by_key(|v| -(*v as i64));
    let mut maxes: Vec<u32> = containers
        .iter()
        .rev()
        .scan(0, |state, v| {
            *state += v;
            Some(*state)
        })
        .collect_vec();
    maxes.reverse();

    (containers, maxes)
}

pub fn solution_1(input: &str) -> String {
    let (containers, maxes) = parse(input);
    fn check_depth(
        containers: &Vec<u32>,
        maxes: &Vec<u32>,
        index: i32,
        current: u32,
        target: u32,
    ) -> u32 {
        let mut count = 0;
        for i in (index + 1) as usize..containers.len() {
            if current + containers[i] > target {
                continue;
            }
            if current + maxes[i] < target {
                break;
            }
            if current + maxes[i] == target {
                count += 1;
                break;
            }

            if current + containers[i] == target {
                count += 1;
            } else {
                count += check_depth(containers, maxes, i as i32, current + containers[i], target);
            }
        }
        count
    }
    check_depth(&containers, &maxes, -1, 0, 150).to_string()
}

pub fn solution_2(input: &str) -> String {
    let (containers, maxes) = parse(input);
    fn check_depth(
        containers: &Vec<u32>,
        maxes: &Vec<u32>,
        index: i32,
        current: u32,
        target: u32,
        combinations: &mut Vec<u32>,
        num: u32,
    ) -> u32 {
        let mut count = 0;
        for i in (index + 1) as usize..containers.len() {
            if current + containers[i] > target {
                continue;
            }
            if current + maxes[i] < target {
                break;
            }
            if current + maxes[i] == target {
                combinations.push(num + (containers.len() - i) as u32);
                count += 1;
                break;
            }

            if current + containers[i] == target {
                combinations.push(num + 1);
                count += 1;
            } else {
                count += check_depth(
                    containers,
                    maxes,
                    i as i32,
                    current + containers[i],
                    target,
                    combinations,
                    num + 1,
                );
            }
        }
        count
    }
    let mut combinations = Vec::new();

    check_depth(&containers, &maxes, -1, 0, 150, &mut combinations, 0);

    let min = combinations.iter().min().unwrap();
    combinations
        .iter()
        .filter(|&v| *v == *min)
        .count()
        .to_string()
}
