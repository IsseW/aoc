#![feature(
    iter_advance_by,
    pattern,
    drain_filter,
    iter_collect_into,
    associated_type_defaults
)]
#![allow(dead_code, unused_variables)]

use chrono::prelude::*;
use itertools::Itertools;
use paste::paste;
use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};
use seq_macro::seq;
use std::{env, fmt::Display, fs};
use std::{path::Path, time::Instant};

mod helpers;

macro_rules! count {
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + count!($($xs)*));
}

macro_rules! years {
    ($($year:literal), *) => {
        paste! {
            $(mod  [< y $year >]; ) *
        }
        const YEAR_COUNT: u32 = count!($($year) *) as u32;
        seq!(N in 1..=25 {
            paste! {
                const PART_1: [fn(&str) -> String; YEAR_COUNT as usize * 25] = [
                    $(
                        #(
                                [< y $year >]::[< d N >]::solution_1,
                        )*
                    )*
                ];
                const PART_2: [fn(&str) -> String; YEAR_COUNT as usize * 25] = [
                    $(
                        #(
                                [< y $year >]::[< d N >]::solution_2,
                        )*
                    )*
                ];
                #()*
            }
        });
    };
}

years!(2015, 2016, 2017, 2018, 2019, 2020, 2021, 2022);

fn create_year_folder(year: u32) {
    let code_folder = format!("./src/y{}", year);
    let input_folder = format!("./input/y{}", year);
    fs::create_dir_all(code_folder.clone()).expect("Could not create folder");
    fs::create_dir_all(input_folder).expect("Could not create folder");
    let mut include = String::new();
    for day in 1..=25 {
        let path = format!("{}/d{}.rs", code_folder.clone(), day);
        let path = Path::new(path.as_str());
        if !path.exists() {
            fs::write(
                path,
                "\npub fn solution_1(input: &str) -> String {\n\t\"Not yet implemented\".into()\n}\n\npub fn solution_2(input: &str) -> String {\n\t\"Not yet implemented\".into()\n}\n"
                    .as_bytes(),
            )
            .expect("Could not create file");
        }

        let path = format!("{}/d{}.rs", code_folder.clone(), day);
        let path = Path::new(path.as_str());
        if !path.exists() {
            fs::write(path, "".as_bytes()).expect("Could not create file");
        }
        include += format!("pub mod d{};\n", day).as_str();
    }

    fs::write(format!("{}/mod.rs", code_folder), include.as_bytes())
        .expect("Could not create file");
}

struct PartResult {
    result: String,
    time: f64,
}

struct DayResult {
    year: u32,
    day: u8,
    read_time: f64,
    part_1: PartResult,
    part_2: PartResult,
}

impl Display for DayResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}\tread time: {}\n\tpart 1:\n\t\tresult: {}\n\t\ttime: {}\n\tpart 2:\n\t\tresult: {}\n\t\ttime: {}", self.year, self.day, self.read_time, self.part_1.result, self.part_1.time, self.part_2.result, self.part_2.time)
    }
}

fn run_day(year: u32, day: u8) -> DayResult {
    let index = (year as usize - 2015) * 25 + day as usize - 1;
    let start = Instant::now();
    let input = fs::read_to_string(format!("./input/y{}/d{}", year, day)).unwrap();
    let input = input.trim().replace('\r', "");
    let read_time = start.elapsed().as_secs_f64();

    let functions = [PART_1[index], PART_2[index]];
    let mut results = functions.iter().map(|f| {
        let start = Instant::now();
        let result = (f)(input.as_str());
        let time = start.elapsed().as_secs_f64();
        PartResult { result, time }
    });

    DayResult {
        year,
        day,
        read_time,
        part_1: results.next().unwrap(),
        part_2: results.next().unwrap(),
    }
}

enum RunKind {
    Year,
    Day,
    All,
}

struct RunState {
    day: u8,
    year: u32,
    create: bool,
    kind: RunKind,
}

impl Default for RunState {
    fn default() -> Self {
        Self {
            day: 255,
            year: 0,
            create: false,
            kind: RunKind::Day,
        }
    }
}

fn main() {
    let state = {
        let mut state = RunState::default();
        let args: Vec<String> = env::args().collect();
        for arg in args {
            let mut bytes = arg.bytes();
            let first = bytes.next();
            if let Some(first) = first {
                if first == b'-' {
                    if matches!(bytes.next().map(|b| b as char), Some('c')) {
                        state.create = true
                    }
                } else if (b'0'..=b'9').contains(&first) {
                    let mut number = (first - b'0') as u32;
                    for byte in bytes {
                        if (b'0'..=b'9').contains(&byte) {
                            number *= 10;
                            number += (byte - b'0') as u32;
                        } else {
                            break;
                        }
                    }
                    match number {
                        0..=25 => state.day = number as u8,
                        2015..=3000 => state.year = number,
                        _ => {}
                    }
                } else {
                    match arg.as_str() {
                        "all" => state.kind = RunKind::All,
                        "year" => state.kind = RunKind::Year,
                        "day" => state.kind = RunKind::Day,
                        _ => {}
                    }
                }
            }
        }
        let date = Utc::now();
        if state.year < 2015 {
            if date.month() < 12 || (date.day() == 1 && date.hour() < 5) {
                state.year = (date.year() - 1) as u32;
            } else {
                state.year = date.year() as u32;
            }
        }
        if state.day > 25 {
            if date.month() < 12 || (date.day() == 1 && date.hour() < 5) || (date.day() > 25) {
                state.day = 25;
            } else if date.hour() < 5 {
                state.day = (date.day() - 1) as u8;
            } else {
                state.day = date.day() as u8;
            }
        }
        state
    };

    if state.create {
        create_year_folder(state.year);
    } else {
        match state.kind {
            RunKind::Day => println!("{}", run_day(state.year, state.day)),
            RunKind::Year => {
                let year = state.year;
                let mut v: Vec<_> = (1..=25)
                    .into_par_iter()
                    .map(|day| run_day(year, day))
                    .collect();
                v.sort_by_key(|r| r.year * 100 + r.day as u32);
                v.iter().for_each(|f| println!("{}", f));
            }
            RunKind::All => {
                let mut v: Vec<_> = (2015..2015 + YEAR_COUNT)
                    .cartesian_product(1..=25)
                    .par_bridge()
                    .map(|(year, day)| run_day(year, day))
                    .collect();
                v.sort_by_key(|r| r.year * 100 + r.day as u32);
                v.iter().for_each(|f| println!("{}", f));
            }
        }
    }
}
