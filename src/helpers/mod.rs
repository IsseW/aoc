pub mod character_set;
mod grid;
mod line;
mod node;
pub mod spiral;
pub mod store;
pub mod volume;

pub use grid::*;
pub use line::*;
pub use node::*;
use num_traits::Zero;

pub fn parse_number<E: std::fmt::Debug, T: std::str::FromStr<Err = E>>(
    chars: &[char],
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

    while *i < chars.len() && chars[*i].is_ascii_digit() {
        t.push(chars[*i]);
        *i += 1;
    }

    t.parse::<T>().expect("Failed to parse number")
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OrderedFloat(pub f64);

impl Eq for OrderedFloat {}

impl PartialOrd for OrderedFloat {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OrderedFloat {
    fn cmp(&self, other: &OrderedFloat) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Add for OrderedFloat {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        OrderedFloat(self.0 + rhs.0)
    }
}

impl Zero for OrderedFloat {
    fn zero() -> Self {
        OrderedFloat(0.0)
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl fmt::Display for OrderedFloat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

use core::str::pattern::Pattern;
use std::{
    fmt,
    ops::Add,
    str::{pattern::ReverseSearcher, CharIndices},
};

pub struct Enclosures<'s, A: Pattern + Copy, B: Pattern + Copy> {
    string: &'s str,
    chars: CharIndices<'s>,
    start: A,
    end: B,
}

impl<'s, A: Pattern + Copy, B: Pattern + Copy> Iterator for Enclosures<'s, A, B> {
    type Item = &'s str;

    fn next(&mut self) -> Option<Self::Item> {
        let s = loop {
            let (i, _) = self.chars.next()?;
            if self.start.is_prefix_of(&self.string[i..]) {
                break i;
            }
        };
        let mut depth = 1;
        let mut has_inner = false;
        let mut chars = self.chars.clone();

        let e = loop {
            let (i, _) = chars.next()?;
            if self.end.is_prefix_of(&self.string[i..]) {
                depth -= 1;
                if depth == 0 {
                    break i;
                }
            } else if self.start.is_prefix_of(&self.string[i..]) {
                has_inner = true;
                depth += 1;
            }
        };

        if !has_inner {
            self.chars = chars;
        }

        self.start.strip_prefix_of(&self.string[s..e])
    }
}

pub trait StrUtil<'s>: Sized {
    type EnclosureIter<A: Copy + Pattern, B: Copy + Pattern>
    where
        Self: 's;
    fn find_enclosures<A: Pattern + Copy, B: Pattern + Copy>(
        self,
        start: A,
        end: B,
    ) -> Self::EnclosureIter<A, B>;

    fn split_once_last<P: Pattern + Copy>(self, p: P) -> Option<(Self, Self)>
    where
        P::Searcher<'s>: ReverseSearcher<'s>;
}

impl<'s> StrUtil<'s> for &'s str {
    type EnclosureIter<A: Copy + Pattern, B: Copy + Pattern> = Enclosures<'s, A, B>;

    fn find_enclosures<A: Pattern + Copy, B: Pattern + Copy>(
        self,
        start: A,
        end: B,
    ) -> Self::EnclosureIter<A, B> {
        Enclosures {
            string: self,
            chars: self.char_indices(),
            start,
            end,
        }
    }

    fn split_once_last<P: Pattern + Copy>(self, p: P) -> Option<(Self, Self)>
    where
        P::Searcher<'s>: ReverseSearcher<'s>,
    {
        self.char_indices()
            .rev()
            .find_map(|(i, _)| p.is_suffix_of(&self[..i]).then_some(i))
            .and_then(|i| Some((p.strip_suffix_of(&self[..i])?, &self[i..])))
    }
}

#[macro_export]
macro_rules! match_starts_with {
    ($string:expr; $($pat:literal @ $ident:ident => $block:block)* _ => $else_block:block) => {
        {
            let _match = $string;
            'match_block: {
                $(
                    if let Some($ident) = _match.strip_prefix($pat) {
                        let res = $block;
                        break 'match_block res;
                    }
                )*
                $else_block
            }
        }
    };
}
