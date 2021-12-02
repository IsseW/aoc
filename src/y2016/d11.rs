use hashbrown::HashMap;
use itertools::Itertools;
use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::helpers::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Kind {
    Generator,
    Chip,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Thing {
    id: NodeId,
    kind: Kind,
}

fn get_floors(input: &str) -> [Vec<Thing>; 4] {
    let mut floors = [Vec::new(), Vec::new(), Vec::new(), Vec::new()];
    input.lines().enumerate().for_each(|(floor, line)| {
        for part in line.split(',') {
            let mut p = part.trim_end_matches('.').split_whitespace().rev();
            match p.next() {
                Some("generator") => {
                    floors[floor].push(Thing {
                        id: NodeId::new(p.next().unwrap()),
                        kind: Kind::Generator,
                    });
                }
                Some("microchip") => {
                    floors[floor].push(Thing {
                        id: NodeId::new(p.next().unwrap().split_once('-').unwrap().0),
                        kind: Kind::Chip,
                    });
                }
                _ => {}
            }
        }
    });
    floors
}

pub fn solution_1(input: &str) -> String {
    let floors = get_floors(input);
    fn solve(floors: [Vec<Thing>; 4], elevator: usize, steps: usize) -> usize {
        if steps > 10 {
            return usize::MAX;
        }
        if floors[0].len() == 0 && floors[1].len() == 0 && floors[2].len() == 0 {
            return steps;
        }
        let mut test = floors[elevator]
            .iter()
            .map(|t| (t.id, 0))
            .collect::<HashMap<_, _>>();

        for thing in &floors[elevator] {
            *test.get_mut(&thing.id).unwrap() += match thing.kind {
                Kind::Generator => 1,
                Kind::Chip => -1,
            };
        }
        if test.values().min().unwrap_or(&0) < &0 {
            return usize::MAX;
        }
        (1..floors[elevator].len())
            .map(|amount| {
                floors[elevator]
                    .iter()
                    .combinations(amount)
                    .par_bridge()
                    .map(|combo| {
                        if elevator > 0 {
                            let mut n_f = floors.clone();
                            let n_e = elevator - 1;
                            n_f[n_e].extend(combo.iter().map(|c| **c));
                            n_f[elevator].drain_filter(|thing| {
                                combo.iter().filter(|t| thing == **t).count() > 0
                            });
                            solve(n_f, elevator - 1, steps + 1)
                        } else {
                            usize::MAX
                        }
                        .min(if elevator < 3 {
                            let mut n_f = floors.clone();
                            let n_e = elevator + 1;
                            n_f[n_e].extend(combo.iter().map(|c| **c));
                            n_f[elevator].drain_filter(|thing| {
                                combo.iter().filter(|t| thing == **t).count() > 0
                            });
                            solve(n_f, elevator + 1, steps + 1)
                        } else {
                            usize::MAX
                        })
                    })
                    .min()
                    .unwrap_or(usize::MAX)
            })
            .min()
            .unwrap_or(usize::MAX)
    }

    solve(floors, 0, 0).to_string()
}

pub fn solution_2(input: &str) -> String {
    "Not yet implemented".into()
}
