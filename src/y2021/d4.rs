use hashbrown::HashMap;

use crate::helpers::*;
const BOARD_SIZE: usize = 5;

fn parse_boards(input: &str, map: &impl Fn(u8) -> u8) -> Vec<Grid<u8>> {
    input
        .split("\n\n")
        .map(|board| {
            board
                .lines()
                .map(|line| {
                    line.split_whitespace()
                        .filter_map(|num| num.parse::<u8>().ok())
                        .map(map)
                })
                .flatten()
                .collect_rows(BOARD_SIZE)
        })
        .collect::<Vec<_>>()
}

// Parses the boards in what order the numbers are called.
fn parse_input(input: &str) -> (Vec<Grid<u8>>, Vec<u8>) {
    let (input, boards) = input.split_once("\n\n").unwrap();
    let inputs: Vec<u8> = input.split(',').map(|l| l.parse().unwrap()).collect();
    let input_order = inputs
        .iter()
        .enumerate()
        .map(|(index, l)| (*l, index as u8))
        .collect::<HashMap<u8, u8>>();
    (parse_boards(boards, &|i| input_order[&i]), inputs)
}

pub fn solution_1(input: &str) -> String {
    let (boards, map) = parse_input(input);

    let (board, value) = boards
        .iter()
        .map(|board| {
            board
                .rows()
                .map(|row| *row.iter().max().unwrap())
                .min()
                .unwrap()
                .min(
                    board
                        .columns()
                        .map(|row| *row.iter().max().unwrap())
                        .min()
                        .unwrap(),
                )
        })
        .enumerate()
        .min_by_key(|(_, v)| *v)
        .unwrap();

    let mut sum = 0;
    for cell in boards[board].iter() {
        if *cell > value {
            sum += map[*cell as usize] as u32;
        }
    }

    (map[value as usize] as u32 * sum).to_string()
}

pub fn solution_2(input: &str) -> String {
    let (boards, map) = parse_input(input);

    let (board, value) = boards
        .iter()
        .map(|board| {
            board
                .rows()
                .map(|row| *row.iter().max().unwrap())
                .min()
                .unwrap()
                .min(
                    board
                        .columns()
                        .map(|row| *row.iter().max().unwrap())
                        .min()
                        .unwrap(),
                )
        })
        .enumerate()
        .max_by_key(|(_, v)| *v)
        .unwrap();

    let mut sum = 0;
    for cell in boards[board].iter() {
        if *cell > value {
            sum += map[*cell as usize] as u32;
        }
    }

    (map[value as usize] as u32 * sum).to_string()
}
