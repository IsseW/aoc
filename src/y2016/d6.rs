use itertools::Itertools;

fn parse(input: &str) -> Vec<Vec<char>> {
    let mut columns = Vec::new();
    input.lines().for_each(|line| {
        line.chars().enumerate().for_each(|(index, char)| {
            if index == columns.len() {
                columns.push(Vec::new())
            }
            columns[index].push(char);
        })
    });
    columns
}

pub fn solution_1(input: &str) -> String {
    let columns = parse(input);
    columns
        .iter()
        .map(|column| {
            *column
                .iter()
                .counts()
                .iter()
                .max_by_key(|(_, &count)| count)
                .unwrap()
                .0
        })
        .collect()
}

pub fn solution_2(input: &str) -> String {
    let columns = parse(input);
    columns
        .iter()
        .map(|column| {
            *column
                .iter()
                .counts()
                .iter()
                .min_by_key(|(_, &count)| count)
                .unwrap()
                .0
        })
        .collect()
}
