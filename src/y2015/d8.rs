pub fn solution_1(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let mut in_mem = 0;
            let chars: Vec<char> = line.trim().chars().collect();
            if chars.len() == 0 {
                return 0;
            }
            let mut i = 1;
            let in_code = chars.len();
            while i < chars.len() - 1 {
                in_mem += 1;
                match chars[i] {
                    '\\' => {
                        i += 1;
                        match chars[i] {
                            'x' => {
                                i += 3;
                            }
                            _ => {
                                i += 1;
                            }
                        }
                    }
                    _ => {
                        i += 1;
                    }
                }
            }

            in_code - in_mem
        })
        .sum::<usize>()
        .to_string()
}

pub fn solution_2(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let mut in_code_new = 2;
            let mut in_code = 0;
            for char in line.trim().chars() {
                in_code += 1;
                match char {
                    '\\' | '"' => {
                        in_code_new += 2;
                    }
                    _ => {
                        in_code_new += 1;
                    }
                }
            }
            if in_code == 0 {
                return 0;
            }
            in_code_new - in_code
        })
        .sum::<usize>()
        .to_string()
}
