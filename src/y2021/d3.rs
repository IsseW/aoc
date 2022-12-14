use crate::helpers;

pub fn solution_1(input: &str) -> String {
    let mut bits = [0; 12];
    let mut num_lines = 0;
    input.lines().for_each(|line| {
        num_lines += 1;
        for (i, char) in line.chars().enumerate() {
            if char == '1' {
                bits[i] += 1;
            }
        }
    });
    let mut a = 0;
    let mut b = 0;
    for (i, bit) in bits.iter().enumerate() {
        if *bit > num_lines / 2 {
            a |= 1 << (11 - i);
        } else {
            b |= 1 << (11 - i);
        }
    }
    (a * b).to_string()
}

pub fn solution_2(input: &str) -> String {
    const LEN: usize = 12;
    let mut numbers: Vec<[bool; LEN]> = Vec::new();
    let mut num_lines = 0;
    input.lines().for_each(|line| {
        num_lines += 1;
        let mut local_bits = [false; LEN];
        for (i, char) in line.chars().enumerate() {
            if char == '1' {
                local_bits[i] = true;
            }
        }
        numbers.push(local_bits);
    });
    let mut numbers2 = numbers.clone();
    for i in 0..LEN {
        let mut count = 0;
        for n in &numbers {
            if n[i] {
                count += 1;
            }
        }
        let all = numbers.len();
        if numbers.len() > 1 {
            numbers.drain_filter(|number| {
                if count >= all - count {
                    !number[i]
                } else {
                    number[i]
                }
            });
        }
        let mut count = 0;
        for n in &numbers2 {
            if n[i] {
                count += 1;
            }
        }
        let all = numbers2.len();
        if numbers2.len() > 1 {
            numbers2.drain_filter(|number| {
                if count >= all - count {
                    number[i]
                } else {
                    !number[i]
                }
            });
        }
    }
    let mut a = 0;
    let mut b = 0;
    for i in 0..LEN {
        a |= i32::from(numbers[0][i]) << (LEN - 1 - i);
        b |= i32::from(numbers2[0][i]) << (LEN - 1 - i);
    }
    (a * b).to_string()
}
