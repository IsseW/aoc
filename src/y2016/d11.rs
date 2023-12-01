use core::fmt;

use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};
use std::collections::HashMap;

use crate::helpers::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Kind {
    Generator,
    Chip,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Thing {
    id: NodeId,
    kind: Kind,
}

impl Thing {
    fn get_counterpart(&self) -> Thing {
        Thing {
            id: self.id,
            kind: match self.kind {
                Kind::Generator => Kind::Chip,
                Kind::Chip => Kind::Generator,
            },
        }
    }
}

const NUM_FLOORS: usize = 4;
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct State {
    current: usize,
    floors: [Vec<Thing>; NUM_FLOORS],
}

impl State {
    fn count_kind(&self, kind: Kind, floor: usize) -> usize {
        self.floors[floor].iter().filter(|t| t.kind == kind).count()
    }
    fn does_thing_exist(&self, floor: usize, thing: Thing) -> bool {
        self.floors[floor].contains(&thing)
    }

    fn get_connecting(&self) -> Vec<State> {
        (1..=2)
            .into_par_iter()
            .map(|amount| {
                self.floors[self.current]
                    .iter()
                    .combinations(amount)
                    .flat_map(|combination| vec![(-1, combination.clone()), (1, combination)])
                    .par_bridge()
                    .filter(move |(dir, combination)| {
                        if self.current == 0 && *dir == -1 || self.current == 3 && *dir == 1 {
                            return false;
                        }
                        if amount == 2 {
                            if *dir == -1 {
                                return false;
                            }
                            if combination[0].kind != combination[1].kind
                                && combination[0].id != combination[1].id
                            {
                                return false;
                            }
                        }
                        let next_floor = (self.current as i32 + dir) as usize;
                        for thing in combination {
                            if !combination.contains(&&thing.get_counterpart()) {
                                match thing.kind {
                                    Kind::Generator => {
                                        if self
                                            .does_thing_exist(self.current, thing.get_counterpart())
                                            && self.count_kind(Kind::Generator, self.current)
                                                > self.count_kind(Kind::Chip, self.current)
                                        {
                                            return false;
                                        }
                                        if !self
                                            .does_thing_exist(next_floor, thing.get_counterpart())
                                            && self.count_kind(Kind::Chip, next_floor)
                                                > self.count_kind(Kind::Generator, next_floor)
                                        {
                                            return false;
                                        }
                                    }
                                    Kind::Chip => {
                                        if self
                                            .does_thing_exist(self.current, thing.get_counterpart())
                                            && self.count_kind(Kind::Chip, self.current)
                                                > self.count_kind(Kind::Generator, self.current)
                                        {
                                            return false;
                                        }
                                        if !self
                                            .does_thing_exist(next_floor, thing.get_counterpart())
                                            && self.count_kind(Kind::Generator, next_floor)
                                                > self.count_kind(Kind::Chip, next_floor)
                                        {
                                            return false;
                                        }
                                    }
                                }
                            }
                        }
                        true
                    })
                    .map(move |(dir, combination)| {
                        let mut new_state = self.clone();
                        new_state.current = (new_state.current as i32 + dir) as usize;
                        new_state.floors[self.current]
                            .retain(|&thing| !combination.iter().any(|&&t| t == thing));
                        new_state.floors[new_state.current].extend(combination.iter().copied());
                        new_state
                    })
            })
            .flatten()
            .collect()
    }

    fn distance(&self) -> usize {
        self.floors
            .iter()
            .enumerate()
            .map(|(index, floor)| (floor.len() / 2 + floor.len() % 2) * (NUM_FLOORS - 1 - index))
            .sum()
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, floor) in self.floors.iter().enumerate().rev() {
            if index == self.current {
                write!(f, "F{} E", index)?;
            } else {
                write!(f, "F{} .", index)?;
            }
            for thing in floor {
                write!(
                    f,
                    " {}{}",
                    match thing.kind {
                        Kind::Generator => 'G',
                        Kind::Chip => 'C',
                    },
                    thing.id.0 % 69
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn get_start_state(input: &str) -> State {
    let mut floors = [Vec::new(), Vec::new(), Vec::new(), Vec::new()];
    input.lines().enumerate().for_each(|(floor, line)| {
        for part in line.split(", ").flat_map(|s| s.split(" and ")) {
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
    State { current: 0, floors }
}

pub fn solution_1(input: &str) -> String {
    let initial_state = get_start_state(input);
    // A star our way to the solution...
    let mut closed = HashMap::new();
    let dis = initial_state.distance();
    const MAX_DISTANCE: usize = 100;
    let mut open_o = HashMap::new();
    let mut open = vec![HashMap::new(); MAX_DISTANCE];
    open[0].insert(initial_state, (0, dis));

    while !open.is_empty() {
        let (closest_state, steps, left) = open
            .iter()
            .find(|x| !x.is_empty())
            .unwrap()
            .iter()
            .next()
            .map(|(state, (steps, left))| (state.clone(), *steps, *left))
            .unwrap();

        if left == 0 {
            return steps.to_string();
        }

        open.iter_mut()
            .find(|x| !x.is_empty())
            .unwrap()
            .remove(&closest_state);
        open_o.remove(&closest_state);

        for state in closest_state.get_connecting() {
            if !closed.contains_key(&state) {
                let distance = state.distance();
                let entry = open_o
                    .entry(state.clone())
                    .or_insert((usize::MAX, distance));
                if entry.0 > steps + 1 {
                    let d = entry.0 + entry.1;
                    let i = d - dis;
                    if i < MAX_DISTANCE {
                        open[i].remove(&state);
                    }
                    entry.0 = steps + 1;
                    let d = entry.0 + entry.1;
                    let i = d - dis;
                    if i < MAX_DISTANCE {
                        open[i].insert(state, *entry);
                    }
                }
            }
        }
        closed.insert(closest_state.clone(), (steps, left));
        if closed.len() % 1000 == 0 {
            println!("{}\t\t{}", closed.len(), open_o.len());
        }
    }
    "No solution found".to_string()
}

pub fn solution_2(input: &str) -> String {
    "Not yet implemented".into()
}
