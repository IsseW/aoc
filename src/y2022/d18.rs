use hashbrown::HashSet;
use itertools::Itertools;
use vek::Vec3;

use crate::helpers::volume::Volume;

fn parse(input: &str) -> Volume<bool> {
    Volume::from_sparse(input.lines().map(|p| {
        Vec3::from(
            p.split(',')
                .map(str::parse)
                .map(Result::unwrap)
                .collect_tuple::<(usize, usize, usize)>()
                .unwrap(),
        )
    }))
    .0
}

const CARDINALS: [Vec3<isize>; 6] = [
    Vec3::new(1, 0, 0),
    Vec3::new(-1, 0, 0),
    Vec3::new(0, 1, 0),
    Vec3::new(0, -1, 0),
    Vec3::new(0, 0, 1),
    Vec3::new(0, 0, -1),
];

pub fn solution_1(input: &str) -> String {
    let volume = parse(input);
    volume
        .iter()
        .map(|(p, t)| {
            if *t {
                CARDINALS
                    .iter()
                    .filter(|n| {
                        let p = p.map2(**n, |p, n| p.checked_add_signed(n));
                        if let Vec3 {
                            x: Some(x),
                            y: Some(y),
                            z: Some(z),
                        } = p
                        {
                            volume.get(Vec3::new(x, y, z)).is_none_or(|p| !*p)
                        } else {
                            true
                        }
                    })
                    .count()
            } else {
                0
            }
        })
        .sum::<usize>()
        .to_string()
}

#[derive(Copy, Clone, Default, PartialEq, Eq)]
enum Cell {
    #[default]
    Water,
    Lava,
    Air,
}

fn floodfill(volume: &mut Volume<Cell>, pos: Vec3<usize>) {
    fn floodfill_inner(
        volume: &Volume<Cell>,
        positions: &mut HashSet<Vec3<usize>>,
        pos: Vec3<usize>,
    ) -> bool {
        if let Some(cell) = volume.get(pos) {
            if *cell != Cell::Water || positions.contains(&pos) {
                return true;
            }
            positions.insert(pos);
            CARDINALS
                .iter()
                .filter_map(|n| {
                    let p = pos.map2(*n, |p, n| p.checked_add_signed(n));
                    Some(Vec3::new(p.x?, p.y?, p.z?))
                })
                .map(|pos| floodfill_inner(volume, positions, pos))
                .reduce(|a, b| a && b)
                .unwrap_or(true)
        } else {
            false
        }
    }

    let mut positions = HashSet::new();

    if floodfill_inner(volume, &mut positions, pos) {
        for p in positions {
            volume[p] = Cell::Air;
        }
    }
}

pub fn solution_2(input: &str) -> String {
    let mut volume = parse(input).map(|b| if b { Cell::Lava } else { Cell::Water });

    for z in 1..volume.size().z - 1 {
        for y in 1..volume.size().y - 1 {
            for x in 1..volume.size().x - 1 {
                floodfill(&mut volume, Vec3::new(x, y, z));
            }
        }
    }

    volume
        .iter()
        .map(|(p, t)| {
            if *t == Cell::Lava {
                CARDINALS
                    .iter()
                    .filter(|n| {
                        let p = p.map2(**n, |p, n| p.checked_add_signed(n));
                        if let Vec3 {
                            x: Some(x),
                            y: Some(y),
                            z: Some(z),
                        } = p
                        {
                            volume
                                .get(Vec3::new(x, y, z))
                                .is_none_or(|p| *p == Cell::Water)
                        } else {
                            true
                        }
                    })
                    .count()
            } else {
                0
            }
        })
        .sum::<usize>()
        .to_string()
}
