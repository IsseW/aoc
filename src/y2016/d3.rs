use itertools::Itertools;

pub fn check_tuple(t: (u32, u32, u32)) -> bool {
    t.0.min(t.1).min(t.2) + t.0.clamp(t.1.min(t.2), t.1).max(t.0.min(t.2)) > t.0.max(t.1).max(t.2)
}

pub fn solution_1(input: &str) -> String {
    input
        .lines()
        .filter_map(|line| line.split_whitespace().collect_tuple())
        .filter_map(|(a, b, c)| {
            Some((
                a.parse().ok()?,
                b.parse().ok()?,
                c.parse().ok()?,
            ))
        })
        .filter(|&t| check_tuple(t))
        .count()
        .to_string()
}

pub fn solution_2(input: &str) -> String {
    input
        .lines()
        .filter_map(|line| line.split_whitespace().collect_tuple())
        .filter_map(|(a, b, c)| {
            Some((
                a.parse().ok()?,
                b.parse().ok()?,
                c.parse().ok()?,
            ))
        })
        .tuples()
        .map(|(t0, t1, t2)| {
            check_tuple((t0.0, t1.0, t2.0)) as u32
                + check_tuple((t0.1, t1.1, t2.1)) as u32
                + check_tuple((t0.2, t1.2, t2.2)) as u32
        })
        .sum::<u32>()
        .to_string()
}
