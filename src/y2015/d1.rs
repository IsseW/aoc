pub fn solution_1(input: &str) -> String {
    input
        .chars()
        .scan(0, |state, char| {
            match char {
                '(' => *state += 1,
                ')' => *state -= 1,
                _ => {}
            }
            Some(*state)
        })
        .last()
        .unwrap()
        .to_string()
}

pub fn solution_2(input: &str) -> String {
    input
        .chars()
        .enumerate()
        .scan(0, |state, (index, char)| {
            match char {
                '(' => *state += 1,
                ')' => *state -= 1,
                _ => {}
            }
            if *state == -1 {
                Some(Some(index + 1))
            } else {
                Some(None)
            }
        })
        .flatten()
        .next()
        .unwrap()
        .to_string()
}
