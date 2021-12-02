use crate::helpers::*;

pub fn solution_1(input: &str) -> String {
    let grid = ('1'..='9').collect_grid_f(3, 3);
    let mut walker = GridWalker::clamped(&grid);
    walker.tp(1, 1);
    let mut code = String::new();
    for line in input.lines() {
        for char in line.chars() {
            match char {
                'U' => walker.up(),
                'D' => walker.down(),
                'L' => walker.left(),
                'R' => walker.right(),
                _ => {}
            }
        }
        code.push(*walker.get());
    }
    code
}

pub fn solution_2(input: &str) -> String {
    let grid = [
        [None, None, Some('1'), None, None],
        [None, Some('2'), Some('3'), Some('4'), None],
        [Some('5'), Some('6'), Some('7'), Some('8'), Some('9')],
        [None, Some('A'), Some('B'), Some('C'), None],
        [None, None, Some('D'), None, None],
    ]
    .into_grid();
    let mut walker = GridWalker::clamped(&grid);
    walker.tp(0, 2);
    walker.collide(|tile| tile.is_none());
    let mut code = String::new();
    for line in input.lines() {
        for char in line.chars() {
            match char {
                'U' => walker.up(),
                'D' => walker.down(),
                'L' => walker.left(),
                'R' => walker.right(),
                _ => {}
            }
        }
        code.push(walker.get().unwrap());
    }
    code
}
