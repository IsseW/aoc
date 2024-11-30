use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

const DIGITS: [u8; 10] = [
    0b0111_0111,
    0b0001_0010,
    0b0101_1101,
    0b0101_1011,
    0b0011_1010,
    0b0110_1011,
    0b0110_1111,
    0b0101_0010,
    0b0111_1111,
    0b0111_1011,
];

const DIGIT_POSITIONS: [&[u8]; 10] = [
    &[0, 1, 2, 4, 5, 6],
    &[2, 5],
    &[0, 2, 3, 4, 6],
    &[0, 2, 3, 5, 6],
    &[1, 2, 3, 5],
    &[0, 1, 3, 5, 6],
    &[0, 1, 3, 4, 5, 6],
    &[0, 2, 5],
    &[0, 1, 2, 3, 4, 5, 6],
    &[0, 1, 2, 3, 5, 6],
];

fn get_possible(segments: &Vec<u8>, possible: &mut [HashSet<u8>; 7]) -> Vec<u8> {
    let possible_digits = (0..10u8)
        .filter(|&digit| segments.len() == DIGIT_POSITIONS[digit as usize].len())
        .filter(|&digit| {
            DIGIT_POSITIONS[digit as usize].iter().all(|segment| true);
            segments.iter().all(|segment| {
                possible[*segment as usize]
                    .iter()
                    .any(|possible| DIGIT_POSITIONS[digit as usize].contains(possible))
            })
        })
        .collect::<Vec<u8>>();
    if possible_digits.is_empty() {
        return possible_digits;
    }
    let possible_segments = possible_digits
        .iter()
        .flat_map(|&digit| DIGIT_POSITIONS[digit as usize].iter())
        .collect::<HashSet<_>>();
    for &segment in segments {
        possible[segment as usize].retain(|seg| possible_segments.contains(seg));
    }
    if possible_digits.len() == 1 {
        // If this is the only segment with this segment that is the only possible connection
        let mut refs: HashMap<u8, Vec<u8>> = HashMap::new();
        for &segment in segments {
            for p in possible[segment as usize].iter() {
                refs.entry(*p).or_default().push(segment);
            }
        }
        for (digit, seg) in refs {
            if seg.len() == 1 {
                possible[seg[0] as usize].retain(|&d| d == digit);
            }
        }
    }

    for i in 2..7 {
        let considered = (0..7)
            .filter(|&i| possible[i].len() == 2)
            .combinations(2)
            .collect::<Vec<_>>();

        for combination in considered {
            if combination
                .iter()
                .skip(1)
                .all(|&i| possible[0].iter().all(|elem| possible[i].contains(elem)))
            {
                let trim = possible[0].clone();
                for i in (0..7).filter(|i| !combination.contains(i)) {
                    possible[i].retain(|elem| !trim.contains(elem));
                }
            }
        }
    }

    possible_digits
}

fn translate(word: &str) -> Vec<u8> {
    word.chars().map(|c| c as u8 - b'a').collect()
}

pub fn solution_1(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let (_, output) = line.split_once(" | ").unwrap();
            output
                .split_whitespace()
                .filter(|word| matches!(word.len(), 2 | 3 | 4 | 7))
                .count()
        })
        .sum::<usize>()
        .to_string()
}

pub fn solution_2(input: &str) -> String {
    let all: HashSet<u8> = (0..7).collect();
    let mut possible: [HashSet<u8>; 7] = [
        all.clone(),
        all.clone(),
        all.clone(),
        all.clone(),
        all.clone(),
        all.clone(),
        all,
    ];
    dbg!(get_possible(&vec![0, 1], &mut possible));
    dbg!(&possible);
    dbg!(get_possible(&vec![0, 2, 3], &mut possible));

    String::new()
    /*
    input
        .lines()
        .map(|line| {
            let (signals, output) = line.split_once(" | ").unwrap();
            let signals = signals
                .split_whitespace()
                .map(translate)
                .collect::<Vec<_>>();
            let output = output.split_whitespace().map(translate).collect::<Vec<_>>();
            let all: HashSet<u8> = (0..7).collect();
            let mut possible: [HashSet<u8>; 7] = [
                all.clone(),
                all.clone(),
                all.clone(),
                all.clone(),
                all.clone(),
                all.clone(),
                all,
            ];
            let mut changed = true;
            while !changed {
                changed = false;
                for signal in &signals {
                    if dbg!(get_possible(signal, &mut possible)).len() > 1 {
                        changed = true;
                    }
                }
                for output in &output {
                    if dbg!(get_possible(output, &mut possible)).len() > 1 {
                        changed = true;
                    }
                }
            }
            output
                .iter()
                .map(|output| (get_possible(output, &mut possible)[0] + '0' as u8) as char)
                .collect::<String>()
        })
        .next()
        .unwrap()
        */
}
