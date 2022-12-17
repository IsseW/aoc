use itertools::Itertools;

use crate::match_starts_with;

#[derive(Debug)]
enum Num {
	Var,
	Const(u64),
}

impl Num {
	fn eval(&self, var: u64) -> u64 {
		match self {
			Num::Var => var,
			Num::Const(i) => *i,
		}
	}
}

#[derive(Default, Debug)]
enum Expr {
	#[default]
	Zero,
	Add(Num, Num),
	Mul(Num, Num),
}

impl Expr {
	fn eval(&self, var: u64) -> u64 {
		match self {
			Expr::Add(a, b) => a.eval(var) + b.eval(var),
			Expr::Mul(a, b) => a.eval(var) * b.eval(var),
    		Expr::Zero => 0,
		}
	}
}

#[derive(Default, Debug)]
struct Monkey {
	items: Vec<u64>,
	oper: Expr,
	test_div: u64,
	throw_true: usize,
	throw_false: usize,
}


fn parse(input: &str) -> Vec<Monkey> {
	input.split("\n\n").map(|monkey| {
		let mut lines = monkey.lines();
		let _index: u32 = strp::parse!(lines.next().unwrap() => "Monkey {}:");
		let mut monkey = Monkey::default();
		for line in lines {
			let line = line.trim_start();
			match_starts_with! { line;
				"Starting items: " @ line => {
					line.split(", ").map(|item| {
						item.parse::<u64>().unwrap()
					}).collect_into(&mut monkey.items);
				}
				"Operation: new = " @ line => {
					let mut items = line.split(' ');
					let first = items.next().unwrap();
					let first = if first == "old" {
						Num::Var
					} else {
						dbg!(first);
						Num::Const(first.parse().unwrap())
					};

					let op = items.next().unwrap();

					let second = items.next().unwrap();
					let second = if second == "old" {
						Num::Var
					} else {
						Num::Const(second.parse().unwrap())
					};

					monkey.oper = match op {
						"*" => Expr::Mul(first, second),
						"+" => Expr::Add(first, second),
						_ => panic!(),
					};
				}
				"Test: divisible by " @ line => {
					monkey.test_div = line.parse().unwrap();
				}
				"If true: throw to monkey " @ line => {
					monkey.throw_true = line.parse().unwrap();
				}
				"If false: throw to monkey " @ line => {
					monkey.throw_false = line.parse().unwrap();
				}
				_ => {
					panic!("Unexpected: {line}");
				}
			}
		}
		monkey
	}).collect()
}

fn calculate_monkey_buisness(input: &str, iter: u64, div: u64) -> u64 {
	let mut monkeys = parse(input);
	let highest = monkeys.iter().fold(1, |acc, monkey| acc * monkey.test_div);
	let mut inspects: Vec<u64> = vec![0; monkeys.len()];
	for _ in 0..iter {
		for i in 0..monkeys.len() {
			let items = monkeys[i].items.drain(..).collect_vec();
			for mut item in items {
				item = monkeys[i].oper.eval(item) / div % highest;
				inspects[i] += 1;
				let to = if item % monkeys[i].test_div == 0 {
					monkeys[i].throw_true
				} else {
					monkeys[i].throw_false
				};
				monkeys[to].items.push(item);
			}
		}
	}
	inspects.sort();
	inspects[inspects.len() - 1] * inspects[inspects.len() - 2]
}

pub fn solution_1(input: &str) -> String {
	calculate_monkey_buisness(input, 20, 3).to_string()
}

pub fn solution_2(input: &str) -> String {
	calculate_monkey_buisness(input, 10000, 1).to_string()
}
