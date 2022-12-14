use core::fmt;
use std::{
    cmp::Ordering,
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use num_traits::Num;

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub struct NodeId(pub u64);

impl NodeId {
    pub fn new(string: &str) -> Self {
        let mut hasher = DefaultHasher::new();
        string.hash(&mut hasher);
        Self(hasher.finish())
    }
}

pub fn binary_search<T: num_traits::PrimInt + fmt::Display, F: FnMut(T) -> Ordering>(
    start: T,
    end: T,
    mut check: F,
) -> T {
    if start >= end {
        return start;
    }
    let center = (start + end) / (T::one() + T::one());
    match check(center) {
        Ordering::Less => binary_search(start, center - T::one(), check),
        Ordering::Greater => binary_search(center + T::one(), end, check),
        Ordering::Equal => center,
    }
}

pub fn extract_numbers<T: Num>(input: &str) -> Vec<T> {
    input
        .split_whitespace()
        .filter_map(|word| {
            T::from_str_radix(
                word.trim_end_matches(|c| matches!(c, '.' | ',' | '?' | '!' | ':' | ';')),
                10,
            )
            .ok()
        })
        .collect()
}
