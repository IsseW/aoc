use itertools::Itertools;

pub fn solution_1(input: &str) -> String {
    let mut values = vec![Vec::new()];
    input
        .lines()
        .map(|line| {
            values[0].clear();
            line.split_whitespace()
                .map(|v| v.parse::<i64>().unwrap())
                .collect_into(&mut values[0]);
            let mut i = 1;
            loop {
                if i == values.len() {
                    values.push(Vec::with_capacity(values[i - 1].len()));
                } else {
                    values[i].clear();
                }
                let [last_values, values] = values.get_many_mut([i - 1, i]).unwrap();
                let mut last = None;
                let mut all_same = true;
                last_values
                    .iter()
                    .copied()
                    .tuple_windows::<(_, _)>()
                    .map(|(a, b)| {
                        let val = b - a;
                        all_same &= val == *last.get_or_insert(val);
                        val
                    })
                    .collect_into(values);
                if all_same {
                    break;
                }

                i += 1;
            }

            for i in (1..=i).rev() {
                let projected = values[i - 1].last().unwrap() + values[i].last().unwrap();
                values[i - 1].push(projected);
            }
            *values[0].last().unwrap()
        })
        .sum::<i64>()
        .to_string()
}

pub fn solution_2(input: &str) -> String {
    let mut values = vec![Vec::new()];
    input
        .lines()
        .map(|line| {
            values[0].clear();
            line.split_whitespace()
                .map(|v| v.parse::<i64>().unwrap())
                .rev()
                .collect_into(&mut values[0]);
            let mut i = 1;
            loop {
                if i == values.len() {
                    values.push(Vec::with_capacity(values[i - 1].len()));
                } else {
                    values[i].clear();
                }
                let [last_values, values] = values.get_many_mut([i - 1, i]).unwrap();
                let mut last = None;
                let mut all_same = true;
                last_values
                    .iter()
                    .copied()
                    .tuple_windows::<(_, _)>()
                    .map(|(a, b)| {
                        let val = b - a;
                        all_same &= val == *last.get_or_insert(val);
                        val
                    })
                    .collect_into(values);
                if all_same {
                    break;
                }

                i += 1;
            }

            for i in (1..=i).rev() {
                let projected = values[i - 1].last().unwrap() + values[i].last().unwrap();
                values[i - 1].push(projected);
            }
            *values[0].last().unwrap()
        })
        .sum::<i64>()
        .to_string()
}
