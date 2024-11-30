pub fn solution_1(input: &str) -> String {
    std::iter::once(None)
        .chain(input.lines().map(|line| {
            Some((
                line,
                line.bytes()
                    .enumerate()
                    .filter_map(|(i, b)| {
                        if !b.is_ascii_digit() && b != b'.' {
                            Some((i, b as char))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>(),
            ))
        }))
        .chain(std::iter::once(None))
        .map_windows(|v: &[_; 3]| {
            (
                v[1].as_ref().expect("Center should always be some").0,
                v.iter().filter_map(|v| v.as_ref()).fold(
                    Vec::<(usize, char)>::new(),
                    |mut v, s| {
                        v.extend(s.1.clone());
                        v
                    },
                ),
            )
        })
        .map(|(line, symbols)| {
            let mut sum = 0;
            let mut line_iter = line.bytes().enumerate().map(|(i, b)| (i, b as char));
            while let Some((i, c)) = line_iter.next() {
                if c.is_ascii_digit() {
                    let mut end = line.len();
                    for (i, c) in line_iter.by_ref() {
                        if !c.is_ascii_digit() {
                            end = i;
                            break;
                        }
                    }
                    let mut symbols = symbols
                        .iter()
                        .filter(|s| (i.saturating_sub(1)..=end).contains(&s.0));

                    // dbg!(&line[i..end]);
                    if symbols.next().is_some() {
                        let num = line[i..end]
                            .parse::<u32>()
                            .expect("This should be a number");
                        sum += num;
                    }
                }
            }
            sum
        })
        .sum::<u32>()
        .to_string()
}

pub fn solution_2(input: &str) -> String {
    let mut symbol_numbers = Vec::new();
    let lines: Vec<_> = std::iter::once(None)
        .chain(input.lines().map(|line| {
            Some((
                line,
                line.bytes()
                    .enumerate()
                    .filter_map(|(i, b)| {
                        if b == b'*' {
                            let l = symbol_numbers.len();
                            symbol_numbers.push(Vec::new());
                            Some((i, b as char, l))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>(),
            ))
        }))
        .chain(std::iter::once(None))
        .map_windows(|v: &[_; 3]| {
            (
                v[1].as_ref().expect("Center should always be some").0,
                v.iter().filter_map(|v| v.as_ref()).fold(
                    Vec::<(usize, char, usize)>::new(),
                    |mut v, s| {
                        v.extend(s.1.clone());
                        v
                    },
                ),
            )
        })
        .collect();
    lines.into_iter().for_each(|(line, symbols)| {
        let mut line_iter = line.bytes().enumerate().map(|(i, b)| (i, b as char));
        while let Some((i, c)) = line_iter.next() {
            if c.is_ascii_digit() {
                let mut end = line.len();
                for (i, c) in line_iter.by_ref() {
                    if !c.is_ascii_digit() {
                        end = i;
                        break;
                    }
                }
                let symbols = symbols
                    .iter()
                    .filter(|s| (i.saturating_sub(1)..=end).contains(&s.0));
                let num = line[i..end]
                    .parse::<u32>()
                    .expect("This should be a number");
                for symbol in symbols {
                    symbol_numbers[symbol.2].push(num);
                }
            }
        }
    });
    symbol_numbers
        .into_iter()
        .filter_map(|v| match *v {
            [a, b] => Some(a * b),
            _ => None,
        })
        .sum::<u32>()
        .to_string()
}
