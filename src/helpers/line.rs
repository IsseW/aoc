use std::ops::RangeInclusive;

use num_traits::AsPrimitive;

use super::OrderedFloat;

pub struct LineIter {
    a: (i32, i32),
    b: (i32, i32),
    current: Option<(i32, i32)>,
}
impl LineIter {
    pub fn new(a: (i32, i32), b: (i32, i32)) -> Self {
        LineIter {
            a,
            b,
            current: None,
        }
    }
}
impl Iterator for LineIter {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = &mut self.current {
            if *current == self.b {
                return None;
            }
            let line_vec = (self.b.0 - self.a.0, self.b.1 - self.a.1);
            let line_len = ((line_vec.0 * line_vec.0 + line_vec.1 * line_vec.1) as f64).sqrt();
            let line_vec = (line_vec.0 as f64 / line_len, line_vec.1 as f64 / line_len);
            let pos = ((current.0 - self.a.0) as f64, (current.1 - self.a.1) as f64);
            let delta = *[
                (line_vec.0.signum() as i32, 0),
                (0, line_vec.1.signum() as i32),
                (line_vec.0.signum() as i32, line_vec.1.signum() as i32),
            ]
            .iter()
            .min_by_key(|delta| {
                let new_pos = (pos.0 + delta.0 as f64, pos.1 + delta.1 as f64);
                let dot = new_pos.0 * line_vec.0 + new_pos.1 * line_vec.1;
                let v = (new_pos.0 - line_vec.0 * dot, new_pos.1 - line_vec.1 * dot);
                OrderedFloat(v.0 * v.0 + v.1 * v.1)
            })
            .unwrap();
            current.0 += delta.0;
            current.1 += delta.1;
        } else {
            self.current = Some(self.a);
        }
        self.current
    }
}

impl From<((i32, i32), (i32, i32))> for LineIter {
    fn from((a, b): ((i32, i32), (i32, i32))) -> Self {
        LineIter::new(a, b)
    }
}

pub fn straight<I>(a: (I, I), b: (I, I)) -> impl Iterator<Item = (I, I)>
where
    I: Ord + std::ops::Sub<I, Output = I> + AsPrimitive<usize>,
    RangeInclusive<I>: Iterator<Item = I> + Clone,
{
    let s = (a.0.min(b.0), a.1.min(b.1));
    let e = (a.0.max(b.0), a.1.max(b.1));
    (s.0..=e.0)
        .cycle()
        .zip((s.1..=e.1).cycle())
        .take((e.0 - s.0).max(e.1 - s.1).as_() + 1)
}
