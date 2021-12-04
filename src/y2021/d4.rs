use crate::helpers::*;
const BOARD_SIZE: usize = 5;

fn parse_boards(input: &str) -> Vec<Grid<u8>> {
    input
        .split("\n\n")
        .map(|board| {
            board
                .lines()
                .map(|line| {
                    line.split_whitespace()
                        .filter_map(|num| num.parse::<u8>().ok())
                })
                .flatten()
                .collect_rows(BOARD_SIZE)
        })
        .collect::<Vec<_>>()
}
fn parse_input(input: &str) -> (Vec<u8>, Vec<Grid<u8>>) {
    let (input, boards) = input.split_once("\n\n").unwrap();

    (
        input.split(',').map(|l| l.parse().unwrap()).collect(),
        parse_boards(boards),
    )
}

pub fn solution_1(input: &str) -> String {
    let (inputs, boards) = parse_input(input);
    let mut board_check = vec![Grid::<bool>::new(BOARD_SIZE, BOARD_SIZE); boards.len()];

    for input in inputs {
        for (board, board_check) in boards.iter().zip(board_check.iter_mut()) {
            for (x, y) in board.indices() {
                if board[(x, y)] == input {
                    board_check[(x, y)] = true;
                    if board_check.get_row(y).iter().all(|&b| b)
                        || board_check.get_column(x).iter().all(|&b| b)
                    {
                        let mut sum = 0;
                        for (x, y) in board.indices() {
                            if !board_check[(x, y)] {
                                sum += board[(x, y)] as u32;
                            }
                        }
                        return (sum * board[(x, y)] as u32).to_string();
                    }
                }
            }
        }
    }
    "No solution found".to_string()
}

pub fn solution_2(input: &str) -> String {
    let (inputs, mut boards) = parse_input(input);
    let mut board_check = vec![Grid::<bool>::new(BOARD_SIZE, BOARD_SIZE); boards.len()];

    for input in inputs {
        if boards.len() == 1 {
            println!("ONE LEFT");
            let board = &boards[0];
            let board_check = &mut board_check[0];

            for (x, y) in board.indices() {
                if board[(x, y)] == input {
                    board_check[(x, y)] = true;
                    if board_check.get_row(y).iter().all(|&b| b)
                        || board_check.get_column(x).iter().all(|&b| b)
                    {
                        let mut sum = 0;
                        for (x, y) in board.indices() {
                            if !board_check[(x, y)] {
                                sum += board[(x, y)] as u32;
                            }
                        }
                        return (sum * board[(x, y)] as u32).to_string();
                    }
                }
            }
        } else {
            let mut b = None;
            for ((index, board), board_check) in
                boards.iter().enumerate().zip(board_check.iter_mut())
            {
                for (x, y) in board.indices() {
                    if board[(x, y)] == input {
                        board_check[(x, y)] = true;
                        if board_check.get_row(y).iter().all(|&b| b)
                            || board_check.get_column(x).iter().all(|&b| b)
                        {
                            b = Some(index);
                        }
                    }
                }
            }
            if let Some(b) = b {
                boards.swap_remove(b);
                board_check.swap_remove(b);
            }
        }
    }

    "No solution found".to_string()
}
