use crate::helpers;

pub fn solution_1(input: &str) -> String {
    let mut last = 0;
    let mut result = 0;
    input.lines().for_each(|r| {
        if let Ok(r) = r.parse() {
            if r > last {
                result += 1;
            }
            last = r;
        }
    });
    (result - 1).to_string()
}

pub fn solution_2(input: &str) -> String {
    let mut last = 0;
    let mut result = 0;
    let lines = input.lines().collect::<Vec<_>>();

    for i in 0..lines.len() - 2 {
        if let (Ok(a), Ok(b), Ok(c)) = (
            lines[i].parse::<i32>(),
            lines[i + 1].parse::<i32>(),
            lines[i + 2].parse::<i32>(),
        ) {
            let r = a + b + c;
            if r > last {
                result += 1;
            }
            last = r;
        }
    }

    (result - 1).to_string()
}
