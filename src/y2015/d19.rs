use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use rand::prelude::IteratorRandom;

pub fn solution_1(input: &str) -> String {
    let (rep, molecule) = input
        .split_once("\n\n")
        .or(input.split_once("\r\n\r\n"))
        .unwrap();
    let molecule = molecule.chars().collect_vec();
    let rep: Vec<(&str, &str)> = rep
        .lines()
        .map(|line| line.split_once(" => ").unwrap())
        .collect();

    let mut set = HashSet::new();

    for (from, to) in rep {
        let mut i = 0;
        'outer: while i < molecule.len() {
            let mut matches = true;
            for (j, char) in from.chars().enumerate() {
                if let Some(c) = molecule.get(i + j) {
                    if char != *c {
                        matches = false;
                        break;
                    }
                } else {
                    break 'outer;
                }
            }
            if matches {
                let mut mol = String::with_capacity(molecule.len() + to.len());
                mol.extend(&molecule[0..i]);
                mol.push_str(to);
                i += from.chars().count();
                mol.extend(&molecule[i..molecule.len()]);
                set.insert(mol);
            } else {
                i += 1;
            }
        }
    }

    set.len().to_string()
}

pub fn solution_2(input: &str) -> String {
    let (rep, org_molecule) = input
        .split_once("\n\n")
        .or(input.split_once("\r\n\r\n"))
        .unwrap();

    let mut molecule = org_molecule.to_string();
    let mut count = 0;

    let replace_map: HashMap<&str, &str> = rep
        .lines()
        .map(|line| {
            let t = line.split_once(" => ").unwrap();
            (t.1, t.0)
        })
        .collect();
    let mut rng = rand::thread_rng();

    let mut last_count = 0;
    let mut tries = 0;

    while molecule != "e" {
        let replace = *replace_map.keys().choose(&mut rng).unwrap();
        while molecule.contains(replace) {
            molecule = molecule.replacen(replace, replace_map[replace], 1);
            count += 1;
        }
        if count == last_count {
            tries += 1;
            if tries > 100 {
                molecule = org_molecule.to_string();
                count = 0;
            }
        } else {
            tries = 0;
            last_count = count;
        }
    }

    count.to_string()
}
