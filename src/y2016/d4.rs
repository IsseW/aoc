use std::cmp;

use itertools::Itertools;

pub fn solution_1(input: &str) -> String {
    input
        .lines()
        .filter_map(|line| {
            let (encrypted, checksum) = line.split_once('[').unwrap();
            let checksum = checksum.trim_end_matches(']');
            let id = if let [.., id] = encrypted.split('-').collect_vec()[..] {
                id.parse::<u32>().unwrap()
            } else {
                panic!();
            };
            let counts = encrypted
                .chars()
                .take_while(|c| !('0'..='9').contains(c))
                .filter(|c| *c != '-')
                .counts();
            let mut counts = counts.iter().collect_vec();
            counts.sort_by(|a, b| match b.1.cmp(a.1) {
                cmp::Ordering::Less => cmp::Ordering::Less,
                cmp::Ordering::Equal => a.0.cmp(b.0),
                cmp::Ordering::Greater => cmp::Ordering::Greater,
            });
            let cs: String = counts.iter().take(5).map(|(&char, _)| char).collect();
            if cs == checksum {
                Some(id)
            } else {
                None
            }
        })
        .sum::<u32>()
        .to_string()
}

fn decrypt(encrypted: &str, id: u32) -> String {
    encrypted
        .bytes()
        .map(|char| {
            if char == b'-' {
                ' '
            } else {
                let letter = char - b'a';
                let shift = id % (b'z' as u32 - b'a' as u32 + 1);
                (b'a' + (letter + shift as u8) % (b'z' - b'a' + 1)) as char
            }
        })
        .collect::<String>()
}

pub fn solution_2(input: &str) -> String {
    input
        .lines()
        .filter_map(|line| {
            let (encrypted, checksum) = line.split_once('[').unwrap();
            let checksum = checksum.trim_end_matches(']');
            let id = if let [.., id] = encrypted.split('-').collect_vec()[..] {
                id.parse().unwrap()
            } else {
                panic!();
            };
            let encrypted: String = encrypted
                .chars()
                .take_while(|c| !(*c >= '0' && *c <= '9'))
                .collect();
            let counts = encrypted.chars().filter(|c| *c != '-').counts();
            let mut counts = counts.iter().collect_vec();
            counts.sort_by(|a, b| match b.1.cmp(a.1) {
                cmp::Ordering::Less => cmp::Ordering::Less,
                cmp::Ordering::Equal => a.0.cmp(b.0),
                cmp::Ordering::Greater => cmp::Ordering::Greater,
            });
            let cs: String = counts.iter().take(5).map(|(&char, _)| char).collect();
            if cs == checksum {
                Some((encrypted, id))
            } else {
                None
            }
        })
        .map(|(encrypted, id)| (decrypt(encrypted.as_str(), id), id))
        .filter_map(|(decrypted, id)| {
            if decrypted.contains("north") && decrypted.contains("pole") {
                Some(id)
            } else {
                None
            }
        })
        .next()
        .unwrap()
        .to_string()
}
