use crate::helpers::*;

pub fn solution_1(input: &str) -> String {
    let chars: Vec<_> = input.chars().collect();
    let mut i = 0;
    let mut len = 0;
    while i < input.len() {
        if chars[i] == '(' {
            i += 1;
            let a: usize = parse_number(&chars, &mut i);
            if chars[i] != 'x' {
                panic!("Expected 'x'");
            }
            i += 1;
            let b: usize = parse_number(&chars, &mut i);
            if chars[i] != ')' {
                panic!("Expected ')'");
            }
            len += a * b;
            i += a + 1;
        } else {
            len += 1;
            i += 1;
        }
    }
    len.to_string()
}

pub fn solution_2(input: &str) -> String {
    fn count(chars: &Vec<char>, i: &mut usize, max: usize) -> usize {
        let mut len = 0;
        while *i < max {
            if chars[*i] == '(' {
                *i += 1;
                let length: usize = parse_number(&chars, i);
                if chars[*i] != 'x' {
                    panic!("Expected 'x'");
                }
                *i += 1;
                let times: usize = parse_number(&chars, i);
                if chars[*i] != ')' {
                    panic!("Expected ')'");
                }
                *i += 1;
                let true_length = count(chars, i, *i + length);
                len += true_length * times;
            } else {
                len += 1;
                *i += 1;
            }
        }
        len
    }
    let chars = input.chars().collect();
    count(&chars, &mut 0, chars.len()).to_string()
}
