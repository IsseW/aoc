use integer_sqrt::IntegerSquareRoot;

pub fn solution_1(input: &str) -> String {
    let input = input.parse().unwrap();
    pub fn count_divisors(number: u128) -> u128 {
        let sqrt = number.integer_sqrt();
        let mut count = 10 + number * 10;
        if number % sqrt == 0 {
            count += sqrt * 10;
        }
        for i in 2..sqrt {
            if number % i == 0 {
                count += (i + number / i) * 10;
            }
        }
        count
    }

    for i in 1..u128::MAX {
        let c = count_divisors(i);
        if c >= input {
            return i.to_string();
        }
    }
    "unexpected".into()
}

pub fn solution_2(input: &str) -> String {
    let input = input.parse().unwrap();
    pub fn count_divisors(number: u128) -> u128 {
        let mut count = number * 11;
        for i in 2..=50 {
            if number % i == 0 {
                count += (number / i) * 11;
            }
        }
        count
    }

    for i in 1..u128::MAX {
        let c = count_divisors(i);
        if c >= input {
            return i.to_string();
        }
    }
    "unexpected".into()
}
