use std::{iter::once, str::Chars, cmp::Ordering};

use crate::helpers;

#[derive(Clone, PartialEq)]
enum Value {
    Integer(u8),
    Array(Vec<Value>),
}

fn parse_value(input: &mut Chars) -> Option<Value> {
    let c = input.next().unwrap();
    match c {
        ']' => None,
        '[' => {
            let mut values = Vec::new();
            while let Some(value) = parse_value(input) {
                values.push(value);
                match input.next().unwrap() {
                    ']' => break,
                    ',' => continue,
                    c => panic!("Unexpected char({c:?})"),
                }
            }
            Some(Value::Array(values))
        }
        _ => {
			let mut peek = input.clone();
			let mut num = String::from(c);
			while let Some(c) = peek.next() {
				if ('0'..='9').contains(&c) {
					*input = peek.clone();
					num.push(c);
				} else {
					break;
				}
			}
			Some(Value::Integer(num.parse().unwrap()))
		},
    }
}

fn parse(input: &str) -> impl Iterator<Item = (Value, Value)> + '_ {
    input.split("\n\n").map(|pair| {
        let (left, right) = pair.split_once('\n').unwrap();
        (
            parse_value(&mut left.chars()).unwrap(),
            parse_value(&mut right.chars()).unwrap(),
        )
    })
}

enum Res {
    Ok,
    Continue,
    Err,
}

fn is_list_ordered<'a>(
    left: impl ExactSizeIterator<Item = &'a Value>,
    right: impl ExactSizeIterator<Item = &'a Value>,
) -> Res {
    let left_len = left.len();
    let right_len = right.len();
    left.zip(right)
        .map(|(left, right)| is_ordered(left, right))
        .find(|res| matches!(res, Res::Ok | Res::Err))
        .unwrap_or(match left_len.cmp(&right_len) {
				Ordering::Less => Res::Ok,
				Ordering::Equal => Res::Continue,
				Ordering::Greater => Res::Err,
		})
}

fn is_ordered(left: &Value, right: &Value) -> Res {
    match (left, right) {
        (Value::Integer(left), Value::Integer(right)) => {
			match left.cmp(right) {
				Ordering::Less => Res::Ok,
				Ordering::Equal => Res::Continue,
				Ordering::Greater => Res::Err,
			}
        }
        (Value::Array(left), Value::Array(right)) => is_list_ordered(left.iter(), right.iter()),
        (Value::Integer(_), Value::Array(right)) => is_list_ordered(once(left), right.iter()),
        (Value::Array(left), Value::Integer(_)) => is_list_ordered(left.iter(), once(right)),
    }
}

pub fn solution_1(input: &str) -> String {
    parse(input)
        .enumerate()
        .filter(|(i, (left, right))| {
            let res = is_ordered(left, right);
            matches!(res, Res::Ok | Res::Continue)
        })
        .map(|(i, _)| i + 1)
        .sum::<usize>()
        .to_string()
}

pub fn solution_2(input: &str) -> String {
	let c = (Value::Array(vec![Value::Array(vec![Value::Integer(2)])]), Value::Array(vec![Value::Array(vec![Value::Integer(6)])]));
	let mut v: Vec<Value> = parse(input).chain(once(c.clone())).flat_map(|(left, right)| {
		[left, right]
	}).collect();

	v.sort_by(|left, right| {
		match is_ordered(left, right) {
			Res::Ok => Ordering::Less,
			Res::Continue => Ordering::Equal,
			Res::Err => Ordering::Greater,
		}
	});

	let mut indices = (0, 0);
	for (i, value) in v.iter().enumerate() {
		if value == &c.0 {
			indices.0 = i + 1;
		} else if value == &c.1 {
			indices.1 = i + 1;
		}
	}

	(indices.0 * indices.1).to_string()
}
