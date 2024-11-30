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
                        .trim_start_matches(['y', '='])
                        .parse::<usize>()
                        .unwrap();
                    parts.next().unwrap();
                    let by = parts.next().unwrap().parse::<i32>().unwrap();
                    let r: Vec<_> = screen.get_row(row).unwrap().iter().copied().collect();
                    let mut row = screen.get_row_mut(row).unwrap();
                    let l = row.len() as i32;
                    for i in 0..l {
                        *row.get_mut(i as usize).unwrap() = r[((i - by + l) % l) as usize];
                    }
                }

                "column" => {
                    let column = parts
                        .next()
                        .unwrap()
                        .trim_start_matches(['x', '='])
                        .parse::<usize>()
                        .unwrap();
                    parts.next().unwrap();
                    let by = parts.next().unwrap().parse::<i32>().unwrap();
                    let r: Vec<_> = screen.get_column(column).unwrap().iter().copied().collect();
                    let mut column = screen.get_column_mut(column).unwrap();
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

    screen.parse_word()
}
