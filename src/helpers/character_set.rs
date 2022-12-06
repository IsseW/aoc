

#[derive(Default, PartialEq, Eq, Hash, Clone, Copy)]
pub struct CharacterSet(u32);

impl CharacterSet {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    pub fn len(&self) -> usize {
        self.0.count_ones() as usize
    }

    pub fn insert(&mut self, mut c: char) {
        c.make_ascii_lowercase();
        assert!(('a'..='z').contains(&c));
        self.0 |= 1 << (c as u8 - b'a');
    }

    pub fn remove(&mut self, mut c: char) {
        c.make_ascii_lowercase();
        assert!(('a'..='z').contains(&c));
        self.0 &= !(1 << (c as u8 - b'a'));
    }

    pub fn contains(&self, mut c: char) -> bool {
        c.make_ascii_lowercase();
        assert!(('a'..='z').contains(&c));
        self.0 & (1 << (c as u8 - b'a')) != 0
    }
}