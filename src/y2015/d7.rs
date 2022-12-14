use hashbrown::HashMap;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct WireId(u16);

impl WireId {
    fn new(id: &str) -> Self {
        let bytes = id.trim().as_bytes();
        Self(((*bytes.first().unwrap_or(&0) as u16) << 8) | (*bytes.get(1).unwrap_or(&0) as u16))
    }
}

#[derive(Clone)]
enum Input {
    Wire(WireId),
    Number(u16),
}

impl Input {
    fn new(string: &str) -> Self {
        if let Ok(num) = string.parse() {
            Self::Number(num)
        } else {
            Self::Wire(WireId::new(string))
        }
    }

    fn get_value(&self, map: &HashMap<WireId, Wire>) -> Option<u16> {
        match self {
            Input::Wire(id) => map.get(id).and_then(|w| w.value),
            Input::Number(n) => Some(*n),
        }
    }
}

#[derive(Clone)]
enum Operator {
    Not(Input),
    Or(Input, Input),
    And(Input, Input),
    LShift(Input, u8),
    RShift(Input, u8),
    Number(Input),
}

impl Operator {
    fn new(string: &str) -> Self {
        let mut split = string.split_whitespace();
        match split.next() {
            Some("NOT") => Self::Not(Input::new(split.next().unwrap())),
            Some(id) => match split.next() {
                Some("AND") => Self::And(Input::new(id), Input::new(split.next().unwrap())),
                Some("OR") => Self::Or(Input::new(id), Input::new(split.next().unwrap())),
                Some("LSHIFT") => Self::LShift(
                    Input::new(id),
                    split.next().unwrap().parse().unwrap(),
                ),
                Some("RSHIFT") => Self::RShift(
                    Input::new(id),
                    split.next().unwrap().parse().unwrap(),
                ),
                None => Self::Number(Input::new(id)),
                _ => panic!(),
            },
            _ => panic!(),
        }
    }

    fn get_value(&self, map: &HashMap<WireId, Wire>) -> Option<u16> {
        match self {
            Operator::Not(a) => a.get_value(map).map(|v| !v),
            Operator::Or(a, b) => a
                .get_value(map)
                .and_then(|a| b.get_value(map).map(|b| a | b)),
            Operator::And(a, b) => a
                .get_value(map)
                .and_then(|a| b.get_value(map).map(|b| a & b)),
            Operator::LShift(a, v) => a.get_value(map).map(|val| val << v),
            Operator::RShift(a, v) => a.get_value(map).map(|val| val >> v),
            Operator::Number(v) => v.get_value(map),
        }
    }
}

#[derive(Clone)]
struct Wire {
    value: Option<u16>,
    input: Operator,
}

fn parse(input: &str) -> HashMap<WireId, Wire> {
    input
        .lines()
        .filter_map(|line| {
            if line.is_empty() {
                return None;
            }
            let split = line.split_once(" -> ").unwrap();
            Some((
                WireId::new(split.1),
                Wire {
                    value: None,
                    input: Operator::new(split.0),
                },
            ))
        })
        .collect()
}

fn run_simulation(map: &HashMap<WireId, Wire>) -> u16 {
    let mut map = (*map).clone();
    let keys: Vec<WireId> = map.keys().copied().collect();
    let a_id = WireId::new("a");
    while map[&a_id].value.is_none() {
        for k in &keys {
            if map[k].value.is_none() {
                let new_val = map[k].input.get_value(&map);
                if let Some(new_val) = new_val {
                    map.get_mut(k).unwrap().value = Some(new_val);
                }
            }
        }
    }
    map[&a_id].value.unwrap()
}

pub fn solution_1(input: &str) -> String {
    let map = parse(input);
    run_simulation(&map).to_string()
}

pub fn solution_2(input: &str) -> String {
    let mut map = parse(input);
    map.get_mut(&WireId::new("b")).unwrap().value = Some(run_simulation(&map));
    run_simulation(&map).to_string()
}
