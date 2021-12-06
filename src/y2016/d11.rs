use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use rayon::iter::{ParallelBridge, ParallelIterator};

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

    fn get_connecting(&self) -> Vec<(State, i32)> {
        (1..=2)
            .map(|amount| {
                self.floors[self.current]
                    .iter()
                    .combinations(amount)
                    .zip(if self.current > 0 && self.current < 3 {
                        [-1, 1].iter()
                    } else if self.current > 0 {
                        [1].iter()
                    } else {
                        [-1].iter()
                    })
                    .filter(move |(combination, &dir)| {
                        if amount == 2 {
                            if dir == -1 {
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
                                                >= self.count_kind(Kind::Chip, self.current)
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
                                            .does_thing_exist(self.current, thing.get_counterpart())
                                            && self.count_kind(Kind::Generator, self.current)
                                                >= self.count_kind(Kind::Chip, self.current)
                                        {
                                            return false;
                                        }
                                    }
                                }
                            }
                        }
                        todo!()
                    })
                    .map(move |(combination, dir)| {
                        let mut new_state = self.clone();
                        new_state.current = (new_state.current as i32 + dir) as usize;
                        new_state.floors[self.current]
                            .drain_filter(|&mut thing| combination.iter().any(|&&t| t == thing));
                        new_state.floors[new_state.current]
                            .extend(combination.iter().map(|t| t.clone()));
                        (new_state, amount as i32 * dir)
                    })
            })
            .flatten()
            .collect()
    }
}

fn get_start_state(input: &str) -> State {
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
    State { current: 0, floors }
}

fn state_distance(state: &State) -> usize {
    state
        .floors
        .iter()
        .enumerate()
        .map(|(index, floor)| floor.len() * (NUM_FLOORS - index - 1))
        .sum()
}

pub fn solution_1(input: &str) -> String {
    let input = "The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
The second floor contains a hydrogen generator.
The third floor contains a lithium generator.
The fourth floor contains nothing relevant.";
    let initial_state = get_start_state(input);
    // A star our way to the solution...
    let mut open = HashMap::new();
    // let mut closed = HashMap::new();
    let dis = state_distance(&initial_state);
    open.insert(initial_state, (0, dis));

    //while open.len() > 0 {
    //    let (closest_state, (steps, left)) = open
    //        .iter()
    //        .min_by_key(|(_, &(steps, left))| steps + left)
    //        .unwrap();
    //    if left == 0 {
    //        return steps.to_string();
    //    }
    //
    //    open.remove(&closest_state);
    //    closed.insert(closest_state.clone(), (steps, left));
    //}
    "No solution found".to_string()
}

pub fn solution_2(input: &str) -> String {
    "Not yet implemented".into()
}
