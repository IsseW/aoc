use itertools::Itertools;

pub fn solution_1(input: &str) -> String {
    let mut input = input.split("\n\n").filter(|s| !s.is_empty());

    let seeds = &input.next().unwrap()["seeds: ".len()..];
    let mut seeds = seeds
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    for map in input {
        let maps = map.lines().skip(1);
        let mut new_seeds = Vec::with_capacity(seeds.len());
        for map in maps {
            let v = map.split_whitespace();
            let (dest, source, len) = v
                .map(|v| v.parse::<u64>().unwrap())
                .collect_tuple()
                .unwrap();

            new_seeds.extend(
                seeds
                    .extract_if(|seed| *seed >= source && *seed < (source + len))
                    .map(|seed: u64| seed.wrapping_add(dest).wrapping_sub(source)),
            )
        }
        seeds.extend(new_seeds);
    }

    seeds.into_iter().min().unwrap().to_string()
}

pub fn solution_2(input: &str) -> String {
    let mut input = input.split("\n\n").filter(|s| !s.is_empty());

    let seeds = &input.next().unwrap()["seeds: ".len()..];
    let mut seeds = seeds
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .array_chunks::<2>()
        .map(|[s, r]| (s, s + r - 1))
        .collect::<Vec<_>>();

    let mut new_seeds = Vec::with_capacity(seeds.len());
    for map in input {
        let maps = map.lines().skip(1);
        for map in maps {
            let v = map.split_whitespace();
            let (dest, source, len) = v
                .map(|v| v.parse::<u64>().unwrap())
                .collect_tuple()
                .unwrap();

            let from_s = source;
            let from_e = source + len - 1;
            let to_s = dest;
            let to_e = dest + len - 1;

            let mut i = 0;
            let mut end = 0;
            while i < seeds.len() - end {
                let (s, e) = seeds[i];
                if from_s <= e && from_e >= s {
                    match (from_s <= s, from_e >= e) {
                        (false, false) => {
                            new_seeds.push((to_s, to_e));
                            seeds.push((from_e + 1, seeds[i].1));
                            seeds[i].1 = from_s - 1;
                            i += 1;
                            end += 1;
                        }
                        (false, true) => {
                            let e = e - from_s;
                            new_seeds.push((to_s, to_s + e));
                            seeds[i].1 = from_s - 1;
                            i += 1;
                        }
                        (true, false) => {
                            let s = s - from_s;
                            new_seeds.push((to_s + s, to_e));
                            seeds[i].0 = from_e + 1;
                            i += 1;
                        }
                        (true, true) => {
                            let s = s - from_s;
                            let e = e - from_s;
                            new_seeds.push((to_s + s, to_s + e));
                            seeds.swap_remove(i);
                            end = end.saturating_sub(1);
                        }
                    }
                } else {
                    i += 1;
                }
            }
        }
        seeds.extend(new_seeds.drain(..));
    }

    seeds.into_iter().map(|(s, _)| s).min().unwrap().to_string()
}
