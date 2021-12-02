use hashbrown::HashMap;

pub fn solution_1(input: &str) -> String {
    input
        .split_whitespace()
        .filter(|&str| {
            let mut last = ' ';
            let mut vowels = 0;
            let mut double = false;
            for char in str.chars() {
                match char {
                    'a' | 'e' | 'i' | 'o' | 'u' => vowels += 1,
                    'b' if last == 'a' => return false,
                    'd' if last == 'c' => return false,
                    'q' if last == 'p' => return false,
                    'y' if last == 'x' => return false,
                    _ => {}
                }
                if char == last {
                    double = true;
                }
                last = char;
            }
            double && vowels >= 3
        })
        .count()
        .to_string()
}

pub fn solution_2(input: &str) -> String {
    input
        .split_whitespace()
        .filter(|&str| {
            let mut last_last = ' ';
            let mut last = ' ';
            let mut pairs = HashMap::with_capacity(str.len() - 1);
            let mut repeat = false;
            let mut drepeat = false;
            for (index, char) in str.chars().enumerate() {
                if !drepeat {
                    let pair = (last, char);
                    if let Some(i) = pairs.get(&pair) {
                        drepeat = *i < index - 1
                    } else {
                        pairs.insert(pair, index);
                    }
                }
                if char == last_last {
                    repeat = true;
                }
                if repeat && drepeat {
                    break;
                }
                last_last = last;
                last = char;
            }
            repeat && drepeat
        })
        .count()
        .to_string()
}
