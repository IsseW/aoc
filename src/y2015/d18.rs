use std::mem;

use crate::helpers::{Grid, Rect};

use num_traits::AsPrimitive;

pub fn solution_1(input: &str) -> String {
    let mut map = Grid::from_map(input);
    let (width, height) = map.get_size();
    let mut m = Grid::new(width, height);
    let mut a = &mut map;
    let mut b = &mut m;
    for _ in 0..100 {
        for (tile, x, y) in a.enumerate() {
            let c = a
                .get_slice(Rect::center(x as i32, y as i32, 1))
                .iter()
                .filter(|&b| *b.unwrap_or(&false))
                .count();
            b[(x, y)] = match c {
                3 => true,
                4 if *tile => true,
                _ => false,
            };
        }
        mem::swap(&mut a, &mut b);
    }
    a.iter().filter(|&&b| b).count().to_string()
}

pub fn solution_2(input: &str) -> String {
    let mut map = Grid::from_map(input);
    let (width, height) = map.get_size();
    let mut m = Grid::new(width, height);
    let mut a = &mut map;
    let mut b = &mut m;
    for _ in 0..100 {
        a[(0, 0)] = true;
        a[(width - 1, 0)] = true;
        a[(0, height - 1)] = true;
        a[(width - 1, height - 1)] = true;
        for (tile, x, y) in a.enumerate() {
            let c = a
                .get_slice(Rect::center(x as i32, y as i32, 1))
                .iter()
                .filter(|&b| *b.unwrap_or(&false))
                .count();
            b[(x, y)] = match c {
                3 => true,
                4 if *tile => true,
                _ => false,
            };
        }
        mem::swap(&mut a, &mut b);
    }
    a[(0, 0)] = true;
    a[(width - 1, 0)] = true;
    a[(0, height - 1)] = true;
    a[(width - 1, height - 1)] = true;
    a.iter().filter(|&&b| b).count().to_string()
}
