mod grid;
mod line;
mod node;
pub mod character_set;

pub use grid::*;
pub use line::*;
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

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct OrderedFloat(pub f32);

impl Eq for OrderedFloat {}

impl Ord for OrderedFloat {
    fn cmp(&self, other: &OrderedFloat) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

use core::str::pattern::Pattern;
use std::{marker::PhantomData, str::{CharIndices, pattern::ReverseSearcher}};

pub struct Enclosures<'s, 'a, A: Pattern<'a> + Copy, B: Pattern<'a> + Copy> {
    string: &'s str,
    chars: CharIndices<'s>,
    start: A,
    end: B,
    _marker: PhantomData<&'a ()>,
}

impl<'s: 'a, 'a, A: Pattern<'a> + Copy, B: Pattern<'a> + Copy> Iterator
    for Enclosures<'s, 'a, A, B>
{
    type Item = &'a str;

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

        Some(self.start.strip_prefix_of(&self.string[s..e])?)
    }
}

pub trait StrUtil<'s>: Sized {
    type EnclosureIter<'a, A: Copy + Pattern<'a>, B: Copy + Pattern<'a>>
    where
        Self: 's;
    fn find_enclosures<'a, A: Pattern<'a> + Copy, B: Pattern<'a> + Copy>(
        self,
        start: A,
        end: B,
    ) -> Self::EnclosureIter<'a, A, B>;

    fn split_once_last<'a, P: Pattern<'a> + Copy>(self, p: P) -> Option<(Self, Self)> where P::Searcher: ReverseSearcher<'a>, 'a: 's, 's: 'a;
}

impl<'s> StrUtil<'s> for &'s str {
    type EnclosureIter<'a, A: Copy + Pattern<'a>, B: Copy + Pattern<'a>> = Enclosures<'s, 'a, A, B>;

    fn find_enclosures<'a, A: Pattern<'a> + Copy, B: Pattern<'a> + Copy>(
        self,
        start: A,
        end: B,
    ) -> Self::EnclosureIter<'a, A, B> {
        Enclosures {
            string: self,
            chars: self.char_indices(),
            start,
            end,
            _marker: PhantomData,
        }
    }

    fn split_once_last<'a, P: Pattern<'a> + Copy>(self, p: P) -> Option<(Self, Self)> where P::Searcher: ReverseSearcher<'a>, 'a: 's, 's: 'a {
        self.char_indices().rev().find_map(|(i, _)| p.is_suffix_of(&self[..i]).then_some(i)).and_then(|i| {
            Some((p.strip_suffix_of(&self[..i])?, &self[i..]))
        })
    }
}