mod grid;
mod node;

pub use grid::*;
pub use node::*;

pub fn parse_number<E: std::fmt::Debug, T: std::str::FromStr<Err = E>>(
    chars: &Vec<char>,
    i: &mut usize,
) -> T {
    let mut t = String::new();
    match chars[*i] {
        '+' => *i += 1,
        '-' => {
            *i += 1;
            t.push('-');
        }
        _ => {}
    }

    while *i < chars.len() && chars[*i].is_digit(10) {
        t.push(chars[*i]);
        *i += 1;
    }

    t.parse::<T>().expect("Failed to parse number")
}
