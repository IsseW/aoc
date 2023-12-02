use enum_map::EnumMap;
use stackvec::StackVec;

#[derive(enum_map::Enum, Clone, Copy, Debug)]
#[repr(u8)]
enum Color {
    Red,
    Green,
    Blue,
}

type Bag = EnumMap<Color, u8>;

struct Game {
    id: u32,
    sets: StackVec<[Bag; 6]>,
}

impl Game {
    fn add_sets(&self) -> Bag {
        self.sets
            .iter()
            .copied()
            .reduce(|a, b| add_colors(a, b))
            .unwrap_or_default()
    }

    fn can_play(&self, bag: Bag) -> bool {
        self.sets.iter().all(|set| contains(*set, bag))
    }

    fn fewest(&self) -> Bag {
        self.sets
            .iter()
            .copied()
            .reduce(|a, b| Bag::from_fn(|color| a[color].max(b[color])))
            .unwrap_or_default()
    }
}

fn contains(bag_a: Bag, bag_b: Bag) -> bool {
    bag_b.iter().all(|(color, c)| bag_a[color] <= *c)
}

fn add_colors(bag_a: Bag, bag_b: Bag) -> Bag {
    EnumMap::from_fn(|color| bag_a[color] + bag_b[color])
}

fn bag_power(bag: Bag) -> u32 {
    bag.iter().map(|(_, p)| *p as u32).product()
}

fn parse_input(input: &str) -> impl Iterator<Item = Game> + '_ {
    input.lines().filter_map(|line| {
        let (_, rest) = line.split_once(' ')?;
        let (id, sets_str) = rest.split_once(':')?;

        let mut sets_chars = sets_str.char_indices();

        let mut sets = StackVec::new();
        let mut bag = Bag::default();

        while let Some((_, c)) = sets_chars.next() {
            debug_assert!(c == ' ', "Expected space, found: {c:?}");
            // First one is a space, so number starts here.
            let (i, _) = sets_chars.next().expect("Expected start of number");
            let mut j = i + 1;
            while let Some((i, c)) = sets_chars.next() {
                if c == ' ' {
                    j = i;
                    break;
                }
            }
            let count = sets_str[i..j]
                .parse::<u8>()
                .expect("Expected a valid number");
            let (n, color) = if let Some((_, c)) = sets_chars.next() {
                match c {
                    'r' => (sets_chars.nth("ed".len()), Color::Red),
                    'g' => (sets_chars.nth("reen".len()), Color::Green),
                    'b' => (sets_chars.nth("lue".len()), Color::Blue),

                    _ => panic!("Expected a start of a color"),
                }
            } else {
                panic!("Expected a start of a color");
            };

            bag[color] += count;
            match n {
                Some((_, ';')) => {
                    sets.try_push(bag)
                        .expect("There shouldn't be more than 6 bags");
                    bag = Bag::default();
                }
                _ => {}
            }
        }
        sets.try_push(bag)
            .expect("There shouldn't be more than 6 bags");

        Some(Game {
            id: id.parse().expect("Unable to parse id"),
            sets,
        })
    })
}

#[allow(unused)]
fn solution_1_speed(input: &str) -> String {
    input
        .lines()
        .filter_map(|line| {
            let (_, rest) = line.split_once(' ')?;
            let (id, sets_str) = rest.split_once(':')?;

            let mut sets_chars = sets_str.char_indices();

            while let Some((_, c)) = sets_chars.next() {
                debug_assert!(c == ' ', "Expected space, found: {c:?}");
                // First one is a space, so number starts here.
                let (i, _) = sets_chars.next()?;
                let mut j = i + 1;
                while let Some((i, c)) = sets_chars.next() {
                    if c == ' ' {
                        j = i;
                        break;
                    }
                }
                let count = sets_str[i..j].parse::<u8>().ok()?;
                if let Some((_, c)) = sets_chars.next() {
                    match c {
                        'r' => {
                            if count <= 12 {
                                sets_chars.nth("ed".len());
                            } else {
                                return None;
                            }
                        }
                        'g' => {
                            if count <= 13 {
                                sets_chars.nth("reen".len());
                            } else {
                                return None;
                            }
                        }
                        'b' => {
                            if count <= 14 {
                                sets_chars.nth("lue".len());
                            } else {
                                return None;
                            }
                        }

                        _ => return None,
                    }
                } else {
                    return None;
                }
            }

            id.parse::<u32>().ok()
        })
        .sum::<u32>()
        .to_string()
}

pub fn solution_1(input: &str) -> String {
    let bag = enum_map::enum_map! {
        Color::Red => 12,
        Color::Green => 13,
        Color::Blue => 14,
    };
    parse_input(input)
        .filter(|game| game.can_play(bag))
        .map(|game| game.id)
        .sum::<u32>()
        .to_string()
}

pub fn solution_2(input: &str) -> String {
    parse_input(input)
        .map(|game| bag_power(game.fewest()))
        .sum::<u32>()
        .to_string()
}
