fn parse_input(input: &str) -> impl Iterator<Item = u32> + '_ {
    input.lines().map(|line| {
        let line = &line["Card ".len()..];
        let (id, numbers) = line.split_once(':').unwrap();
        let numbers = &numbers[1..];

        let mut winning = [false; 256];

        let mut wins = 0;
        let mut iter = numbers.split_whitespace();
        for number in iter.by_ref() {
            if number == "|" {
                break;
            } else {
                let i = number.parse::<usize>().unwrap();
                winning[i] = true;
            }
        }
        for number in iter {
            let i = number.parse::<usize>().unwrap();
            wins += winning[i] as u32;
        }

        wins
    })
}

pub fn solution_1(input: &str) -> String {
    parse_input(input)
        .map(|wins| if wins == 0 { 0 } else { 1 << (wins - 1) })
        .sum::<u64>()
        .to_string()
}

pub fn solution_2(input: &str) -> String {
    let mut counts = [1; 11];
    let mut i = 0;
    let mut total: u64 = 0;
    for wins in parse_input(input) {
        let j = i % counts.len();
        let c = counts[j];
        total += c;
        counts[j] = 1;
        i += 1;
        for j in i..i + wins as usize {
            counts[j % counts.len()] += c;
        }
    }
    total.to_string()
}
