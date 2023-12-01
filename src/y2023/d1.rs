fn sum_first_and_last_digits(
    input: &str,
    get_digit: impl Fn((usize, char), bool) -> Option<u32>,
) -> String {
    input
        .lines()
        .filter_map(|input| {
            let mut iter = input.char_indices().filter_map(|t| get_digit(t, false));
            let first = iter.next()?;
            let mut iter = input
                .char_indices()
                .rev()
                .filter_map(|t| get_digit(t, true));
            let last = iter.next()?;

            Some((first, last))
        })
        .map(|(a, b)| a * 10 + b)
        .sum::<u32>()
        .to_string()
}

pub fn solution_1(input: &str) -> String {
    sum_first_and_last_digits(input, |(_, c), _| c.to_digit(10))
}

pub fn solution_2(input: &str) -> String {
    sum_first_and_last_digits(input, |(i, c), reversed| {
        c.to_digit(10).or_else(|| {
            let check = |t| {
                if reversed {
                    input[..=i].ends_with(t)
                } else {
                    input[i..].starts_with(t)
                }
            };
            [
                "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
            ]
            .iter()
            .position(check)
            .map(|i| i as u32 + 1)
        })
    })
}
