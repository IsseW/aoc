use bitvec::array::BitArray;

#[derive(Clone, Copy)]
struct Card {
    count: u32,
    wins: u32,
}

fn parse_input(input: &str) -> impl Iterator<Item = Card> + '_ {
    input.lines().map(|line| {
        let line = &line["Card ".len()..];
        let (id, numbers) = line.split_once(':').unwrap();
        let numbers = &numbers[1..];

        let mut winning = BitArray::<[u64; 4]>::default();
        let mut wins = 0;
        let mut iter = numbers.split_whitespace();
        while let Some(number) = iter.next() {
            if number == "|" {
                break;
            } else {
                let i = number.parse::<usize>().unwrap();
                winning.set(i, true);
            }
        }
        for number in iter {
            let i = number.parse::<usize>().unwrap();
            wins += winning[i] as u32;
        }

        Card { count: 1, wins }
    })
}

pub fn solution_1(input: &str) -> String {
    parse_input(input)
        .map(|card| {
            if card.wins == 0 {
                0
            } else {
                1 << (card.wins - 1)
            }
        })
        .sum::<u64>()
        .to_string()
}

pub fn solution_2(input: &str) -> String {
    let mut cards: Vec<_> = parse_input(input).collect();
    for i in 0..cards.len() {
        let card = cards[i];
        for i in i + 1..(i + 1 + card.wins as usize).min(cards.len()) {
            cards[i].count += card.count;
        }
    }
    cards
        .into_iter()
        .map(|card| card.count)
        .sum::<u32>()
        .to_string()
    // let mut counts = vec![1; cards.len()];
    // for (i, card) in cards.iter().enumerate() {
    //     let wins = (card.ours & card.winning).count_ones();
    //     let count = counts[i];
    //     for v in &mut counts[i + 1..(i + 1 + wins).min(cards.len())] {
    //         *v += count;
    //     }
    // }
    // counts.iter().sum::<u128>().to_string()
}
