use hashbrown::HashSet;

fn is_abba(str: &str) -> bool {
    if str.len() >= 4 {
        let str = str.as_bytes();
        for i in 0..str.len() - 3 {
            if str[i] == str[i + 3] && str[i + 1] == str[i + 2] && str[i] != str[i + 1] {
                return true;
            }
        }
        false
    } else {
        false
    }
}

fn collect_aba(str: &str) -> HashSet<(u8, u8)> {
    let mut set = HashSet::new();
    let str = str.as_bytes();
    for i in 0..str.len() - 2 {
        if str[i] == str[i + 2] && str[i] != str[i + 1] {
            set.insert((str[i], str[i + 1]));
        }
    }
    set
}

pub fn solution_1(input: &str) -> String {
    input
        .lines()
        .filter(|line| {
            let w = line.split(['[', ']']);
            let within = w.clone().skip(1).step_by(2).any(is_abba);
            let outside = w.step_by(2).any(is_abba);
            !within && outside
        })
        .count()
        .to_string()
}

pub fn solution_2(input: &str) -> String {
    input
        .lines()
        .filter(|line| {
            let w = line.split(['[', ']']);
            let inside = w
                .clone()
                .skip(1)
                .step_by(2)
                .map(collect_aba)
                .reduce(|a, b| a.union(&b).copied().collect())
                .unwrap();
            let outside = w
                .step_by(2)
                .map(collect_aba)
                .reduce(|a, b| a.union(&b).copied().collect())
                .unwrap()
                .iter()
                .copied()
                .collect();
            inside.intersection(&outside).count() > 0
        })
        .count()
        .to_string()
}
