use hashbrown::HashMap;


#[derive(Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
struct Monkey(u32);

const ROOT: Monkey = Monkey(0);
const HUMAN: Monkey = Monkey(1);

#[derive(Clone, Copy)]
enum Formula {
	Const(i64),
	Add(Monkey, Monkey),
	Sub(Monkey, Monkey),
	Mul(Monkey, Monkey),
	Div(Monkey, Monkey),
}

impl Formula {
	fn eval(&self, monkeys: &Monkeys) -> i64 {
		match *self {
			Formula::Const(c) => c,
			Formula::Add(a, b) => monkeys.eval(a) + monkeys.eval(b),
			Formula::Sub(a, b) => monkeys.eval(a) - monkeys.eval(b),
			Formula::Mul(a, b) => monkeys.eval(a) * monkeys.eval(b),
			Formula::Div(a, b) => monkeys.eval(a) / monkeys.eval(b),
		}
	}

	fn maybe_eval(&self, monkeys: &Monkeys) -> Option<i64> {
		match *self {
			Formula::Const(c) => Some(c),
			Formula::Add(a, b) => Some(monkeys.maybe_eval(a)? + monkeys.maybe_eval(b)?),
			Formula::Sub(a, b) => Some(monkeys.maybe_eval(a)? - monkeys.maybe_eval(b)?),
			Formula::Mul(a, b) => Some(monkeys.maybe_eval(a)? * monkeys.maybe_eval(b)?),
			Formula::Div(a, b) => Some(monkeys.maybe_eval(a)? / monkeys.maybe_eval(b)?),
		}
	}
}

struct Monkeys {
	monkeys: Vec<Formula>,
}

impl Monkeys {
	fn eval(&self, monkey: Monkey) -> i64 {
		self.monkeys[monkey.0 as usize].eval(self)
	}

	fn maybe_eval(&self, monkey: Monkey) -> Option<i64> {
		if monkey == HUMAN {
			None
		} else {
			self.monkeys[monkey.0 as usize].maybe_eval(self)
		}
	}

	fn solve(&self, monkey: Monkey, should_equal: i64) -> i64 {
		match monkey {
			ROOT => {
				let formula = self.monkeys[monkey.0 as usize];
				match formula {
					Formula::Add(a, b) | Formula::Sub(a, b) | Formula::Mul(a, b) | Formula::Div(a, b) => {
						self.maybe_eval(a).map(|a| self.solve(b, a)).or_else(|| {
							self.maybe_eval(b).map(|b| self.solve(a, b))
						}).unwrap()
					},
					Formula::Const(_) => panic!("No"),
				}
			},
			HUMAN => should_equal,
			_ => {
				let formula = self.monkeys[monkey.0 as usize];
				match formula {
					Formula::Const(_) => panic!("No"),
					Formula::Add(a, b) => self.maybe_eval(a).map(|f| self.solve(b, should_equal - f)).or_else(|| {
						self.maybe_eval(b).map(|f| self.solve(a, should_equal - f))
					}).unwrap(),
					Formula::Sub(a, b) => self.maybe_eval(a).map(|f| self.solve(b, f - should_equal)).or_else(|| {
						self.maybe_eval(b).map(|f| self.solve(a, should_equal + f))
					}).unwrap(),
					Formula::Mul(a, b) => self.maybe_eval(a).map(|f| self.solve(b, should_equal / f)).or_else(|| {
						self.maybe_eval(b).map(|f| self.solve(a, should_equal / f))
					}).unwrap(),
					Formula::Div(a, b) => self.maybe_eval(a).map(|f| self.solve(b, f / should_equal)).or_else(|| {
						self.maybe_eval(b).map(|f| self.solve(a, should_equal * f))
					}).unwrap(),
				}
			}
		}
	}

	fn get(&self, monkey: Monkey) -> Formula {
		self.monkeys[monkey.0 as usize]
	}
}

fn parse<'a>(input: &'a str) -> Monkeys {
	let mut monkey_map = HashMap::new();
	monkey_map.insert("root", ROOT);
	monkey_map.insert("humn", HUMAN);
	let mut idx = monkey_map.len() as u32;
	let mut get_monkey = |monkey: &'a str| -> Monkey {
		*monkey_map.entry(monkey).or_insert_with(|| {
			let i = idx;
			idx += 1;
			Monkey(i)
		})
	};


	let mut monkeys = Vec::new();

	for input in input.lines() {
		let Some((monkey, input)) = input.split_once(": ") else {
			panic!()
		};
		
		let formula = {
			let mut input = input.split(" ");
			if let Some(a) = input.next() {
				if let (Some(op), Some(b)) = (input.next(), input.next()) {
					let a = get_monkey(a);
					let b = get_monkey(b);

					match op {
						"+" => Formula::Add(a, b),
						"-" => Formula::Sub(a, b),
						"*" => Formula::Mul(a, b),
						"/" => Formula::Div(a, b),
						_ => panic!(),
					}

				} else if let Ok(num) = a.parse::<i64>() {
					Formula::Const(num)
				} else {
					panic!()
				}
			} else {
				panic!()
			}
		};

		let monkey = get_monkey(monkey);

		monkeys.push((monkey, formula));
	}

	monkeys.sort_by_key(|(monkey, _)| *monkey);
	Monkeys {
		monkeys: monkeys.into_iter().map(|(_, f)| f).collect(),
	}
}

pub fn solution_1(input: &str) -> String {
	let monkeys = parse(input);

	monkeys.eval(ROOT).to_string()
}

pub fn solution_2(input: &str) -> String {
	let monkeys = parse(input);

	monkeys.solve(ROOT, 0).to_string()
}
