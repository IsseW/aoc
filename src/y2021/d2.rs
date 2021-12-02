use crate::helpers;

pub fn solution_1(input: &str) -> String {
    let mut x = 0;
    let mut y = 0;
    input.lines().for_each(|line| {
        let (dir, amount) = line.split_once(' ').unwrap();
        let amount = amount.parse::<i32>().unwrap();

        match dir {
            "forward" => x += amount,
            "down" => y += amount,
            "up" => y -= amount,
            _ => {}
        }
    });

    (x * y).to_string()
}

pub fn solution_2(input: &str) -> String {
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;
    input.lines().for_each(|line| {
        let (dir, amount) = line.split_once(' ').unwrap();
        let amount = amount.parse::<i32>().unwrap();

        match dir {
            "forward" => {
                x += amount;
                y += amount * aim;
            }
            "down" => aim += amount,
            "up" => aim -= amount,
            _ => {}
        }
    });

    (x * y).to_string()
}
