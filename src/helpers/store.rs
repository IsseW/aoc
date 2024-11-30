use std::{fmt, hash::Hash, marker::PhantomData};

pub struct Id<T> {
    idx: u32,
    phantom: PhantomData<T>,
}

impl<T> Id<T> {
    fn new(idx: u32) -> Self {
        Self {
            idx,
            phantom: PhantomData,
        }
    }
}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Id<T> {}

impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.idx == other.idx
    }
}

impl<T> Eq for Id<T> {}

impl<T> PartialOrd for Id<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for Id<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.idx.cmp(&other.idx)
    }
}

impl<T> Hash for Id<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.idx.hash(state);
    }
}

impl<T> fmt::Debug for Id<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Id").field(&self.idx).finish()
    }
}

#[derive(Clone)]
pub struct Store<T> {
    items: Vec<T>,
}

impl<T> Store<T> {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn insert(&mut self, item: T) -> Id<T> {
        let id = Id::new(self.items.len() as u32);
        self.items.push(item);
        id
    }

    pub fn insert_mut(&mut self, item: T) -> (Id<T>, &mut T) {
        let id = Id::new(self.items.len() as u32);
        self.items.push(item);
        (id, self.items.last_mut().expect("We just pushed an item"))
    }

    pub fn construct(&mut self, f: impl FnOnce(Id<T>) -> T) -> Id<T> {
        let id = Id::new(self.items.len() as u32);
        self.items.push(f(id));
        id
    }

    pub fn get(&self, id: Id<T>) -> &T {
        self.items.get(id.idx as usize).unwrap()
    }

    pub fn get_mut(&mut self, id: Id<T>) -> &mut T {
        self.items.get_mut(id.idx as usize).unwrap()
    }
}
