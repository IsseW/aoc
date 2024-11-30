use std::{collections::VecDeque, mem};

use enumset::EnumSet;
use strum::{EnumIter, IntoEnumIterator};
use vek::Vec2;

#[derive(enumset::EnumSetType, EnumIter, Debug)]
enum Dir {
    North,
    South,
    West,
    East,
}

impl Dir {
    fn vec(&self) -> Vec2<i32> {
        match self {
            Dir::North => -Vec2::unit_y(),
            Dir::South => Vec2::unit_y(),
            Dir::West => -Vec2::unit_x(),
            Dir::East => Vec2::unit_x(),
        }
    }

    fn is_positive(&self) -> bool {
        match self {
            Dir::North => true,
            Dir::South => false,
            Dir::West => false,
            Dir::East => true,
        }
    }

    fn neg(&self) -> Dir {
        match self {
            Dir::North => Dir::South,
            Dir::South => Dir::North,
            Dir::West => Dir::East,
            Dir::East => Dir::West,
        }
    }

    fn select(&self, v: Vec2<i32>) -> i32 {
        match self {
            Dir::North | Dir::South => v.y,
            Dir::West | Dir::East => v.x,
        }
    }
}

pub fn solution_1(input: &str) -> String {
    let mut start_pos = Vec2::zero();
    let mut width = None;
    let height = input
        .lines()
        .enumerate()
        .filter(|(y, l)| {
            let w = width.get_or_insert(l.len());
            if let Some((x, _)) = l.bytes().enumerate().find(|(_, b)| *b == b'S') {
                start_pos = Vec2::new(x, *y).as_::<i32>();
            }
            l.len() == *w
        })
        .count();
    let width = width.unwrap();
    let index = |p: Vec2<i32>| {
        if p.x < 0 || p.x >= width as i32 || p.y < 0 || p.y >= height as i32 {
            return None;
        }
        Some(p.y as usize * (width + 1) + p.x as usize)
    };
    let index_counts = |p: Vec2<i32>| p.y as usize * width + p.x as usize;

    let mut counts = vec![0u16; width * height];
    counts[index_counts(start_pos)] = 1;
    let mut positions: VecDeque<_> = Dir::iter()
        .map(|d| (d.neg(), start_pos + d.vec()))
        .collect();
    while let Some((came_from, pos)) = positions.pop_back() {
        let Some(i) = index(pos) else {
            continue;
        };
        let b = input.as_bytes()[i];
        let mut set = match b {
            b'|' => Dir::South | Dir::North,
            b'-' => Dir::East | Dir::West,
            b'L' => Dir::North | Dir::East,
            b'J' => Dir::North | Dir::West,
            b'7' => Dir::South | Dir::West,
            b'F' => Dir::South | Dir::East,
            b'.' => EnumSet::new(),
            b'S' => EnumSet::new(),

            _ => panic!("Unexpected character, {:?}, pos: {:?}", b as char, pos),
        };
        if !set.remove(came_from) {
            continue;
        }

        let new_count = counts[index_counts(pos + came_from.vec())] + 1;
        let i = index_counts(pos);

        if counts[i] != 0 {
            if counts[i] == new_count || counts[i] == new_count - 1 {
                return (new_count - 1).to_string();
            }
            if counts[i] < new_count {
                continue;
            }
        }
        counts[i] = new_count;
        for dir in set {
            positions.push_front((dir.neg(), pos + dir.vec()));
        }
    }
    "No solution found".to_string()
}

pub fn solution_2(input: &str) -> String {
    let mut width = None;
    let start_pos = input
        .lines()
        .enumerate()
        .find_map(|(y, l)| {
            let w = width.get_or_insert(l.len());
            l.bytes()
                .enumerate()
                .find(|(_, b)| *b == b'S')
                .map(|(x, _)| Vec2::new(x, y).as_::<i32>())
        })
        .expect("There should be a start node");
    let width = width.unwrap();
    let height = input.len() / width;
    let index = |p: Vec2<i32>| {
        if p.x < 0 || p.x >= width as i32 || p.y < 0 || p.y >= height as i32 {
            return None;
        }
        Some(p.y as usize * (width + 1) + p.x as usize)
    };
    let input = input.as_bytes();
    let mut dirs = Dir::iter();
    let mut next_dir = || {
        dirs.find(|dir| {
            let p = start_pos + dir.vec();
            if let Some(i) = index(p) {
                matches!(
                    (input[i], dir.neg()),
                    (b'|', Dir::South | Dir::North)
                        | (b'-', Dir::East | Dir::West)
                        | (b'L', Dir::North | Dir::East)
                        | (b'J', Dir::North | Dir::West)
                        | (b'7', Dir::South | Dir::West)
                        | (b'F', Dir::South | Dir::East)
                )
            } else {
                false
            }
        })
        .expect("There should be at least one connection to the start node")
    };
    let mut last_dir;
    let mut dir = next_dir();
    let first_dir = dir;
    let mut current = start_pos;

    let mut path_area = 0;

    let mut first_corner = None;
    let mut last_corner = None::<(Vec2<i32>, Vec2<i32>)>;

    let mut area = (0, 0);

    let mut add_area = |last: (Vec2<i32>, Vec2<i32>), current: (Vec2<i32>, Vec2<i32>)| {
        area.0 += (last.0.y + current.0.y) * (last.0.x - current.0.x);
        area.1 += (last.1.y + current.1.y) * (last.1.x - current.1.x);
    };

    loop {
        current += dir.vec();
        last_dir = dir;

        let i = index(current).expect("Path should stay in bounds");
        let b = input[i];
        let mut is_start = false;
        let mut set = match b {
            b'|' => Dir::South | Dir::North,
            b'-' => Dir::East | Dir::West,
            b'L' => Dir::North | Dir::East,
            b'J' => Dir::North | Dir::West,
            b'7' => Dir::South | Dir::West,
            b'F' => Dir::South | Dir::East,
            b'S' => {
                is_start = true;
                last_dir.neg() | first_dir
            }

            _ => panic!(
                "Unexpected character connection, {:?}, pos: {:?}",
                b as char, current
            ),
        };
        assert!(set.remove(last_dir.neg()), "Wrong enter direction");
        dir = set.into_iter().next().expect("No direction to choose from");

        if last_dir != dir {
            let mut current = if last_dir.is_positive() ^ dir.is_positive() {
                (current + Vec2::unit_x(), current + Vec2::unit_y())
            } else {
                (current, current + 1)
            };
            if let Some(last) = last_corner {
                if dir.select(last.0) != dir.select(current.0) {
                    mem::swap(&mut current.0, &mut current.1);
                }
                add_area(last, current);
            } else {
                first_corner = Some(current);
            }
            last_corner = Some(current);
        }
        path_area += 1;

        if is_start {
            break;
        }
    }

    let last = last_corner.unwrap();
    let current = first_corner.unwrap();
    add_area(last, current);

    let big_area = area.0.abs().max(area.1.abs()) / 2;
    let area = area.0.abs().min(area.1.abs()) / 2;

    assert_eq!(area, big_area - path_area);
    area.to_string()
}
