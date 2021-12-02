use hashbrown::HashSet;

pub fn solution_1(input: &str) -> String {
    input
        .split(", ")
        .map(|instr| {
            let mut chars = instr.chars();
            let dir = chars.next().unwrap();

            let amount = i32::from_str_radix(chars.collect::<String>().as_str(), 10).unwrap();

            (dir, amount)
        })
        .scan(((0, 1), (0, 0)), |(dir, pos), (rot, amount)| {
            match rot {
                'L' => {
                    let temp = dir.1;
                    dir.1 = dir.0;
                    dir.0 = -temp;
                }
                'R' => {
                    let temp = dir.1;
                    dir.1 = -dir.0;
                    dir.0 = temp;
                }
                _ => panic!(),
            }
            pos.0 += dir.0 * amount;
            pos.1 += dir.1 * amount;

            Some(pos.0.abs() + pos.1.abs())
        })
        .last()
        .unwrap()
        .to_string()
}

pub fn solution_2(input: &str) -> String {
    let mut dir: (i32, i32) = (0, 1);
    let mut pos = (0, 0);
    let mut visited = HashSet::new();
    for (rot, amount) in input.split(", ").map(|instr| {
        let mut chars = instr.chars();
        let dir = chars.next().unwrap();

        let amount = i32::from_str_radix(chars.collect::<String>().as_str(), 10).unwrap();

        (dir, amount)
    }) {
        match rot {
            'L' => {
                let temp = dir.1;
                dir.1 = dir.0;
                dir.0 = -temp;
            }
            'R' => {
                let temp = dir.1;
                dir.1 = -dir.0;
                dir.0 = temp;
            }
            _ => panic!(),
        }
        for _ in 0..amount {
            visited.insert(pos);
            pos.0 += dir.0;
            pos.1 += dir.1;
            if visited.contains(&pos) {
                return (pos.0.abs() + pos.1.abs()).to_string();
            }
        }
    }

    "Unexptected".into()
}
