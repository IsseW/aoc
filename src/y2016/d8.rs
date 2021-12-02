use crate::helpers::*;

fn parse_screen(input: &str) -> Grid<bool> {
    let mut screen = Grid::new(50, 6);
    input.lines().for_each(|line| {
        let mut parts = line.split_whitespace();
        match parts.next().unwrap() {
            "rect" => {
                let (width, height) = parts.next().unwrap().split_once('x').unwrap();
                if let (Ok(width), Ok(height)) = (width.parse::<usize>(), height.parse::<usize>()) {
                    screen
                        .get_slice_mut(Rect::new(0, 0, width, height))
                        .set_all(true);
                }
            }
            "rotate" => match parts.next().unwrap() {
                "row" => {
                    let row = parts
                        .next()
                        .unwrap()
                        .trim_start_matches(|a| a == 'y' || a == '=')
                        .parse::<usize>()
                        .unwrap();
                    parts.next().unwrap();
                    let by = parts.next().unwrap().parse::<i32>().unwrap();
                    let r: Vec<_> = screen.get_row(row).iter().map(|b| *b).collect();
                    let mut row = screen.get_row_mut(row);
                    let l = row.len() as i32;
                    for i in 0..l {
                        *row.get_mut(i as usize).unwrap() = r[((i - by + l) % l) as usize];
                    }
                }

                "column" => {
                    let column = parts
                        .next()
                        .unwrap()
                        .trim_start_matches(|a| a == 'x' || a == '=')
                        .parse::<usize>()
                        .unwrap();
                    parts.next().unwrap();
                    let by = parts.next().unwrap().parse::<i32>().unwrap();
                    let r: Vec<_> = screen.get_column(column).iter().map(|b| *b).collect();
                    let mut column = screen.get_column_mut(column);
                    let l = column.len() as i32;
                    for i in 0..l {
                        *column.get_mut(i as usize).unwrap() = r[((i - by + l) % l) as usize];
                    }
                }

                _ => {}
            },

            _ => {}
        }
    });
    screen
}

pub fn solution_1(input: &str) -> String {
    parse_screen(input)
        .iter()
        .filter(|b| **b)
        .count()
        .to_string()
}

pub fn solution_2(input: &str) -> String {
    let screen = parse_screen(input);
    // println!("{}", screen.to_map());
    let mut result = String::new();
    for i in 0..10 {
        let x = i * 5;

        let slice = screen.get_slice(Rect::new(x, 0, 4, 6));
        let v: Vec<_> = slice
            .iter()
            .map(|b| {
                if let Some(b) = b {
                    if *b {
                        1
                    } else {
                        0
                    }
                } else {
                    0
                }
            })
            .collect();
        let mut w = 0u32;
        for i in 0..v.len() {
            w |= v[i] << i;
        }

        // println!("{}", w);

        result.push(match w {
            10090902 => 'A',
            7968663 => 'B',
            1120031 => 'F',
            15323542 => 'G',
            10067865 => 'H',
            6916236 => 'J',
            6920598 => 'O',
            1145239 => 'P',
            7889182 => 'S',
            6920601 => 'U',
            15803535 => 'Z',
            _ => '_',
        })
    }

    result
}
