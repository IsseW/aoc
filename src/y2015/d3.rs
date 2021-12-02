use hashbrown::HashSet;

pub fn solution_1(input: &str) -> String {
    let mut set = HashSet::new();
    let mut pos = (0, 0);
    set.insert(pos);
    for char in input.chars() {
        match char {
            '<' => pos.0 -= 1,
            '>' => pos.0 += 1,
            'v' => pos.1 -= 1,
            '^' => pos.1 += 1,
            _ => {}
        }
        set.insert(pos);
    }
    set.len().to_string()
}

pub fn solution_2(input: &str) -> String {
    let mut set = HashSet::new();
    set.insert((0, 0));
    let mut poses = [(0, 0), (0, 0)];
    let mut i = 0;
    for char in input.bytes().map(|byte| byte as char) {
        let pos = &mut poses[i];
        match char {
            '<' => pos.0 -= 1,
            '>' => pos.0 += 1,
            'v' => pos.1 -= 1,
            '^' => pos.1 += 1,
            _ => {}
        }
        set.insert(*pos);
        i += 1;
        i %= poses.len();
    }
    set.len().to_string()
}
