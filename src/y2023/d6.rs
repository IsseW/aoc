fn parse_u64(input: &mut &str, mut should_stop: impl FnMut(char) -> bool) -> Option<u64> {
    let mut val = 0;
    let mut started = false;
    for (i, c) in input.bytes().enumerate().map(|(i, c)| (i, c as char)) {
        match c {
            '0'..='9' => {
                started = true;
                val *= 10;
                val += (c as u8 - b'0') as u64
            }
            _ => {
                if started && should_stop(c) {
                    *input = &(*input)[i..];
                    return Some(val);
                }
            }
        }
    }

    if started {
        *input = "";
        Some(val)
    } else {
        None
    }
}

/// Solution for d = s * (t - s)
fn wins_for(time: u64, distance: u64) -> u64 {
    let d = distance as f64 + 1.0;
    let t = time as f64;
    let r = (t * t - 4.0 * d).sqrt();
    let s_min = (t - r) / 2.0;
    let s_max = (t + r) / 2.0;

    let s_min = s_min.ceil() as u64;
    let s_max = s_max.floor() as u64;

    s_max - s_min + 1
}

pub fn solution_1(input: &str) -> String {
    let (time, distance) = input.split_once('\n').unwrap();
    let mut time = &time["Time: ".len()..];
    let mut distance = &distance["Distance: ".len()..];

    let mut product = 1;
    while let Some((time, distance)) =
        parse_u64(&mut time, |_| true).zip(parse_u64(&mut distance, |_| true))
    {
        product *= wins_for(time, distance);
    }

    product.to_string()
}

pub fn solution_2(input: &str) -> String {
    let (time, distance) = input.split_once('\n').unwrap();
    let time = parse_u64(&mut &time["Time: ".len()..], |_| false).unwrap();
    let distance = parse_u64(&mut &distance["Distance: ".len()..], |_| false).unwrap();

    wins_for(time, distance).to_string()
}
