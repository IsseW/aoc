use hashbrown::{HashMap, HashSet};

use crate::helpers;

enum Target {
    Bot(u32),
    Output(u32),
}

struct Transfer {
    from: u32,
    low: Target,
    high: Target,
}

fn run_bots(input: &str) -> (HashMap<u32, HashSet<u32>>, HashMap<u32, HashSet<u32>>) {
    let mut bots: HashMap<u32, HashSet<u32>> = HashMap::new();
    let mut outputs: HashMap<u32, HashSet<u32>> = HashMap::new();
    let mut transfers = Vec::new();
    input.lines().for_each(|line| {
        let mut parts = line.split_whitespace();
        match parts.next() {
            Some("value") => {
                let value = parts.next().unwrap().parse::<u32>().unwrap();
                parts.advance_by(3).unwrap();
                let bot = parts.next().unwrap().parse::<u32>().unwrap();
                if let Some(b) = bots.get_mut(&bot) {
                    b.insert(value);
                } else {
                    let mut c = HashSet::new();
                    c.insert(value);
                    bots.insert(bot, c);
                }
            }
            Some("bot") => {
                let from = parts.next().unwrap().parse::<u32>().unwrap();
                parts.advance_by(3).unwrap();
                let type_a = parts.next().unwrap();
                let id_a = parts.next().unwrap().parse::<u32>().unwrap();
                parts.advance_by(3).unwrap();
                let type_b = parts.next().unwrap();
                let id_b = parts.next().unwrap().parse::<u32>().unwrap();
                // println!(
                //     "bot:{} low->{}:{}, high->{}:{}",
                //     from, type_a, id_a, type_b, id_b
                // );
                transfers.push(Transfer {
                    from,
                    low: if type_a == "bot" {
                        Target::Bot(id_a)
                    } else {
                        Target::Output(id_a)
                    },
                    high: if type_b == "bot" {
                        Target::Bot(id_b)
                    } else {
                        Target::Output(id_b)
                    },
                });
            }

            _ => {}
        }
    });

    let mut changed = true;
    while changed {
        changed = false;
        for transfer in &transfers {
            if bots.contains_key(&transfer.from) && bots.get_mut(&transfer.from).unwrap().len() == 2
            {
                let low = bots
                    .get_mut(&transfer.from)
                    .unwrap()
                    .iter()
                    .copied()
                    .min()
                    .unwrap();
                match transfer.low {
                    Target::Bot(id) => {
                        if let Some(b) = bots.get_mut(&id) {
                            if b.insert(low) {
                                changed = true;
                            }
                        } else {
                            let mut c = HashSet::new();
                            c.insert(low);
                            bots.insert(id, c);
                            changed = true;
                        }
                    }
                    Target::Output(id) => {
                        if let Some(b) = outputs.get_mut(&id) {
                            b.insert(low);
                        } else {
                            let mut c = HashSet::new();
                            c.insert(low);
                            outputs.insert(id, c);
                        }
                    }
                }
                let high = bots
                    .get_mut(&transfer.from)
                    .unwrap()
                    .iter()
                    .copied()
                    .max()
                    .unwrap();
                match transfer.high {
                    Target::Bot(id) => {
                        if let Some(b) = bots.get_mut(&id) {
                            if b.insert(high) {
                                changed = true;
                            }
                        } else {
                            let mut c = HashSet::new();
                            c.insert(high);
                            bots.insert(id, c);
                            changed = true;
                        }
                    }
                    Target::Output(id) => {
                        if let Some(b) = outputs.get_mut(&id) {
                            b.insert(high);
                        } else {
                            let mut c = HashSet::new();
                            c.insert(high);
                            outputs.insert(id, c);
                        }
                    }
                }
            }
        }
    }
    (bots, outputs)
}

pub fn solution_1(input: &str) -> String {
    let (bots, _) = run_bots(input);
    let mut id = 0;
    for (key, bot) in &bots {
        if bot.contains(&17) && bot.contains(&61) {
            id = *key;
            break;
        }
    }
    id.to_string()
}

pub fn solution_2(input: &str) -> String {
    let (_, outputs) = run_bots(input);
    outputs[&0]
        .iter()
        .chain(outputs[&1].iter())
        .chain(outputs[&2].iter())
        .copied()
        .reduce(|a, b| a * b)
        .unwrap()
        .to_string()
}
