fn parse(input: &str) -> Vec<[i64; 5]> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split(',');
            [
                split.next().unwrap().split_whitespace().nth(2).unwrap().parse().unwrap(),
                split.next().unwrap().split_whitespace().nth(1).unwrap().parse().unwrap(),
                split.next().unwrap().split_whitespace().nth(1).unwrap().parse().unwrap(),
                split.next().unwrap().split_whitespace().nth(1).unwrap().parse().unwrap(),
                split.next().unwrap().split_whitespace().nth(1).unwrap().parse().unwrap(),
            ]
        })
        .collect()
}

pub fn solution_1(input: &str) -> String {
    let input = parse(input);
    let mut max = 0;
    for i in 0..100 {
        for j in 0..100 - i {
            for k in 0..100 - i - j {
                let h = 100 - i - j - k;
                let a = input[0][0] * i + input[1][0] * j + input[2][0] * k + input[3][0] * h;
                let b = input[0][1] * i + input[1][1] * j + input[2][1] * k + input[3][1] * h;
                let c = input[0][2] * i + input[1][2] * j + input[2][2] * k + input[3][2] * h;
                let d = input[0][3] * i + input[1][3] * j + input[2][3] * k + input[3][3] * h;

                max = if a <= 0 || b <= 0 || c <= 0 || d <= 0 {
                    0
                } else {
                    a * b * c * d
                }
                .max(max);
            }
        }
    }
    max.to_string()
}

pub fn solution_2(input: &str) -> String {
    let input = parse(input);
    let mut max = 0;
    for i in 0..100 {
        for j in 0..100 - i {
            for k in 0..100 - i - j {
                let h = 100 - i - j - k;
                let a = input[0][0] * i + input[1][0] * j + input[2][0] * k + input[3][0] * h;
                let b = input[0][1] * i + input[1][1] * j + input[2][1] * k + input[3][1] * h;
                let c = input[0][2] * i + input[1][2] * j + input[2][2] * k + input[3][2] * h;
                let d = input[0][3] * i + input[1][3] * j + input[2][3] * k + input[3][3] * h;
                let e = input[0][4] * i + input[1][4] * j + input[2][4] * k + input[3][4] * h;

                if e != 500 {
                    continue;
                }

                max = if a <= 0 || b <= 0 || c <= 0 || d <= 0 {
                    0
                } else {
                    a * b * c * d
                }
                .max(max);
            }
        }
    }
    max.to_string()
}
