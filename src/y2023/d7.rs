use itertools::Itertools;

struct Hand {
    hand: [u8; 13],
    hand_order: [u8; 5],
    bid: u64,
}

pub fn solution_1(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let (hand_input, bid) = line.split_once(' ').unwrap();
            let mut hand = [0; 13];
            let mut hand_order = [0; 5];
            for (j, char) in hand_input.bytes().enumerate() {
                let i = match char {
                    b'2'..=b'9' => char - b'2',
                    b'T' => 8,
                    b'J' => 9,
                    b'Q' => 10,
                    b'K' => 11,
                    b'A' => 12,
                    _ => panic!(),
                };
                hand_order[j] = i;
                hand[i as usize] += 1;
            }

            Hand {
                hand,
                hand_order,
                bid: bid.parse::<u64>().unwrap(),
            }
        })
        .map(|hand| {
            let mut highest_count = 0;
            let mut second_count = 0;
            for (card, count) in hand.hand.into_iter().enumerate() {
                if count > highest_count {
                    second_count = highest_count;

                    highest_count = count;
                } else if count > second_count {
                    second_count = count;
                }
            }

            let score = match (second_count, highest_count) {
                (1, 1) => 0,
                (1, 2) => 1,
                (2, 2) => 2,
                (1, 3) => 3,
                (2, 3) => 4,
                (1, 4) => 5,
                (0, 5) => 6,
                _ => panic!(),
            };

            (score, hand)
        })
        .sorted_by_key(|(k, hand)| (*k, hand.hand_order))
        .enumerate()
        .map(|(i, (_, hand))| (i as u64 + 1) * hand.bid)
        .sum::<u64>()
        .to_string()
}

pub fn solution_2(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let (hand_input, bid) = line.split_once(' ').unwrap();
            let mut hand = [0; 13];
            let mut hand_order = [0; 5];
            for (j, char) in hand_input.bytes().enumerate() {
                let i = match char {
                    b'J' => 0,
                    b'2'..=b'9' => char - b'2' + 1,
                    b'T' => 9,
                    b'Q' => 10,
                    b'K' => 11,
                    b'A' => 12,
                    _ => panic!(),
                };
                hand_order[j] = i;
                hand[i as usize] += 1;
            }

            Hand {
                hand,
                hand_order,
                bid: bid.parse::<u64>().unwrap(),
            }
        })
        .map(|hand| {
            let mut highest_count = 0;
            let mut second_count = 0;
            for (card, count) in hand.hand.into_iter().enumerate().skip(1) {
                if count > highest_count {
                    second_count = highest_count;

                    highest_count = count;
                } else if count > second_count {
                    second_count = count;
                }
            }
            highest_count += hand.hand[0];

            let score = match (second_count, highest_count) {
                (1, 1) => 0,
                (1, 2) => 1,
                (2, 2) => 2,
                (1, 3) => 3,
                (2, 3) => 4,
                (1, 4) => 5,
                (0, 5) => 6,
                _ => panic!(),
            };

            (score, hand)
        })
        .sorted_by_key(|(k, hand)| (*k, hand.hand_order))
        .enumerate()
        .map(|(i, (_, hand))| (i as u64 + 1) * hand.bid)
        .sum::<u64>()
        .to_string()
}
