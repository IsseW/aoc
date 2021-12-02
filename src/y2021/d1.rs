use crate::helpers;

pub fn solution_1(input: &str) -> String {
    let mut last = 0;
    let mut result = 0;
    input.lines().for_each(|r| {
        if let Ok(r) = i32::from_str_radix(r, 10) {
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
            i32::from_str_radix(lines[i], 10),
            i32::from_str_radix(lines[i + 1], 10),
            i32::from_str_radix(lines[i + 2], 10),
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
