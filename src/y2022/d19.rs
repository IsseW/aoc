use num_traits::SaturatingSub;
use rayon::prelude::{ParallelBridge, ParallelIterator};
use vek::Vec4;

// x = ore,
// y = clay,
// z = obsidian,
// w = geode,
#[derive(Default, Clone)]
struct Factory {
    rates: Vec4<u32>,
    amounts: Vec4<u32>,
    max_crafted: Option<usize>,

    minutes_left: u32,
}

impl Factory {
    fn new(minutes: u32) -> Self {
        Self {
            rates: Vec4::unit_x(),
            amounts: Vec4::zero(),
            max_crafted: None,
            minutes_left: minutes,
        }
    }
    fn finish(mut self) -> Self {
        self.amounts += self.rates * self.minutes_left;
        self.minutes_left = 0;
        self
    }
}

#[derive(Default)]
struct Blueprint {
    costs: Vec4<Vec4<u32>>,

    max_costs: Vec4<u32>,
}

fn get_robot<'a, T>(v: &'a Vec4<T>, s: &str) -> &'a T {
    match s {
        "ore" => &v.x,
        "clay" => &v.y,
        "obsidian" => &v.z,
        "geode" => &v.w,
        _ => unreachable!(),
    }
}

fn get_robot_mut<'a, T>(v: &'a mut Vec4<T>, s: &str) -> &'a mut T {
    match s {
        "ore" => &mut v.x,
        "clay" => &mut v.y,
        "obsidian" => &mut v.z,
        "geode" => &mut v.w,
        _ => panic!("Unexpected key: {s}"),
    }
}

fn parse(input: &str) -> impl Iterator<Item = Blueprint> + '_ {
    input.lines().map(|line| {
        let (_, rest) = line.split_once(": ").unwrap();
        let mut blueprint = Blueprint::default();

        for robot in rest.split(". ") {
            let mut w = robot.split_whitespace();
            w.next();
            let robot = w.next().unwrap();
            w.next();
            w.next();
            let mut cost = Vec4::zero();
            while let Some(c) = w.next() {
                let ty = w.next().unwrap().trim_end_matches('.');
                *get_robot_mut(&mut cost, ty) = c.parse().unwrap();
                w.next();
            }
            *get_robot_mut(&mut blueprint.costs, robot) = cost;
        }

        blueprint.max_costs = blueprint
            .costs
            .reduce(|a, b| a.map2(b, |a, b| a.max(b)))
            .with_w(u32::MAX);

        blueprint
    })
}

fn compare_aqc(a: Vec4<Option<u32>>, b: Vec4<Option<u32>>) -> bool {
    a.iter()
        .zip(b.iter())
        .rev()
        .find_map(|(a, b)| match (a, b) {
            (Some(a), Some(b)) => Some(a >= b),
            (Some(_), None) => Some(true),
            (None, Some(_)) => Some(false),
            (None, None) => None,
        })
        .unwrap()
}

fn solve_r(factory: Factory, blueprint: &Blueprint) -> u32 {
    let start = factory.max_crafted.map_or(0, |i| i.saturating_sub(1));
    let mut max_score = factory.clone().finish().amounts.w;
    for i in start..4 {
        let cost = blueprint.costs[i];
        if factory.rates[i] >= blueprint.max_costs[i] {
            continue;
        }
        if factory.amounts[i]
            >= (blueprint.max_costs * Vec4::new(2, 2, 1, 1) / Vec4::new(1, 1, 1, 1))[i]
        {
            continue;
        }
        let Some(turns) = cost
            .saturating_sub(&factory.amounts)
            .zip(factory.rates)
            .into_array()
            .try_map(|(a, b)| {
                if a == 0 {
                    Some(0)
                } else if b == 0 {
                    None
                } else {
                    Some(a.div_ceil(b))
                }
            })
        else {
            continue;
        };

        let turns = turns.into_iter().max().unwrap();
        if turns + 1 + ((3 - i) as u32) * 2 >= factory.minutes_left {
            continue;
        }

        let mut factory = factory.clone();
        factory.max_crafted = Some(factory.max_crafted.map_or(i, |d| d.max(i)));
        factory.minutes_left -= turns;
        factory.amounts += factory.rates * turns;
        factory.amounts -= cost;

        factory.minutes_left -= 1;
        factory.amounts += factory.rates;
        factory.rates[i] += 1;

        max_score = max_score.max(solve_r(factory, blueprint));
    }

    max_score
}

fn solve(factory: Factory, blueprint: &Blueprint) -> u32 {
    let mut factories = vec![factory.clone()];
    let mut best_factory = factory;
    let mut ext = Vec::new();
    while !factories.is_empty() {
        for factory in factories.iter() {
            let factory = factory.clone().finish();
            if best_factory.amounts.w < factory.amounts.w {
                best_factory = factory;
            }
        }
        factories
            .drain(..)
            .flat_map(|factory| {
                blueprint
                    .costs
                    .iter()
                    .enumerate()
                    .skip(factory.max_crafted.map_or(0, |i| i.saturating_sub(1)))
                    .filter_map(move |(i, cost)| {
                        if factory.rates[i] >= blueprint.max_costs[i]
                            || factory.amounts[i]
                                >= (blueprint.max_costs * Vec4::new(2, 3, 1, 1)
                                    / Vec4::new(1, 2, 1, 1))[i]
                        {
                            return None;
                        }
                        let turns = cost
                            .saturating_sub(&factory.amounts)
                            .zip(factory.rates)
                            .into_array()
                            .try_map(|(a, b)| {
                                if a == 0 {
                                    Some(0)
                                } else if b == 0 {
                                    None
                                } else {
                                    Some(a.div_ceil(b))
                                }
                            })?
                            .into_iter()
                            .max()
                            .unwrap();

                        if turns + 1 + (i != 3) as u32 * 3 >= factory.minutes_left {
                            return None;
                        }
                        Some((turns, cost, i))
                    })
                    .map(move |(turns, cost, i)| {
                        let mut factory = factory.clone();
                        factory.max_crafted = Some(factory.max_crafted.map_or(i, |d| d.max(i)));
                        factory.minutes_left -= turns;
                        factory.amounts += factory.rates * turns;
                        factory.amounts -= *cost;

                        factory.minutes_left -= 1;
                        factory.amounts += factory.rates;

                        factory.rates[i] += 1;
                        factory
                    })
            })
            .collect_into(&mut ext);

        factories.append(&mut ext);
    }
    best_factory.amounts.w
}

pub fn solution_1(input: &str) -> String {
    let def_factory = Factory::new(24);
    parse(input)
        .enumerate()
        .map(|(i, blueprint)| {
            let id = (i + 1) as u32;
            solve_r(def_factory.clone(), &blueprint) * id
        })
        .sum::<u32>()
        .to_string()
}

pub fn solution_2(input: &str) -> String {
    let def_factory = Factory::new(32);
    let res = parse(input)
        .take(3)
        .par_bridge()
        .map(|blueprint| solve_r(def_factory.clone(), &blueprint))
        .product::<u32>()
        .to_string();

    res
}
