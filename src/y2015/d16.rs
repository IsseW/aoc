#[derive(Default)]
struct Sue {
    number: u32,
    data: [Option<u32>; 10],
}

impl Sue {
    fn new(input: &str) -> Self {
        let mut s = Self::default();
        let (n, d) = input.split_once(": ").unwrap();
        s.number = u32::from_str_radix(n.split_once(' ').unwrap().1, 10).unwrap();

        let split = d.split(", ");

        for d in split {
            let (thing, number) = d.split_once(':').unwrap();
            s.data[match thing {
                "children" => 0,
                "cats" => 1,
                "samoyeds" => 2,
                "pomeranians" => 3,
                "akitas" => 4,
                "vizslas" => 5,
                "goldfish" => 6,
                "trees" => 7,
                "cars" => 8,
                "perfumes" => 9,
                _ => panic!(),
            }] = Some(u32::from_str_radix(number.trim(), 10).unwrap());
        }
        s
    }

    fn equals_1(&self, other: &[u32; 10]) -> bool {
        for i in 0..10 {
            if let Some(value) = self.data[i] {
                if value != other[i] {
                    return false;
                }
            }
        }
        true
    }

    fn equals_2(&self, other: &[u32; 10]) -> bool {
        for i in 0..10 {
            if let Some(value) = self.data[i] {
                if !match i {
                    1 | 7 => value > other[i],
                    3 | 6 => value < other[i],
                    _ => value == other[i],
                } {
                    return false;
                }
            }
        }
        true
    }
}

pub fn solution_1(input: &str) -> String {
    let sue = [3, 7, 2, 3, 0, 0, 5, 3, 2, 1];
    for line in input.lines() {
        let n = Sue::new(line);
        if n.equals_1(&sue) {
            return n.number.to_string();
        }
    }

    "Error".into()
}

pub fn solution_2(input: &str) -> String {
    let sue = [3, 7, 2, 3, 0, 0, 5, 3, 2, 1];
    for line in input.lines() {
        let n = Sue::new(line);
        if n.equals_2(&sue) {
            return n.number.to_string();
        }
    }

    "Error".into()
}
