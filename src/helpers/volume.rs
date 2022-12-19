use std::ops::{Index, IndexMut};

use vek::Vec3;

pub struct Volume<T> {
    data: Vec<T>,
    size: Vec3<usize>,
}

impl<T> Volume<T> {
    pub fn new(size: Vec3<usize>) -> Self
    where
        T: Default + Clone,
    {
        Volume {
            data: vec![T::default(); size.x * size.y * size.z],
            size,
        }
    }
    pub unsafe fn get_unchecked(&self, p: Vec3<usize>) -> &T {
        self.data
            .get_unchecked((p.z * self.size.y + p.y) * self.size.x + p.x)
    }
    pub unsafe fn get_unchecked_mut(&mut self, p: Vec3<usize>) -> &mut T {
        self.data
            .get_unchecked_mut((p.z * self.size.y + p.y) * self.size.x + p.x)
    }

    pub fn get(&self, p: Vec3<usize>) -> Option<&T> {
        self.size
            .map2(p, |a, b| b < a)
            .reduce_and()
            .then(|| unsafe { self.get_unchecked(p) })
    }
    pub fn get_mut(&mut self, p: Vec3<usize>) -> Option<&mut T> {
        self.size
            .map2(p, |a, b| b < a)
            .reduce_and()
            .then(|| unsafe { self.get_unchecked_mut(p) })
    }

    pub fn iter(&self) -> impl Iterator<Item = (Vec3<usize>, &T)> + '_ + Clone {
        (0..self.size.z)
            .flat_map(move |z| {
                (0..self.size.y)
                    .flat_map(move |y| (0..self.size.x).map(move |x| Vec3::new(x, y, z)))
            })
            .map(|p| (p, unsafe { self.get_unchecked(p) }))
    }

    pub fn map<U>(self, f: impl FnMut(T) -> U) -> Volume<U> {
        Volume {
            data: self.data.into_iter().map(f).collect(),
            size: self.size,
        }
    }

    pub fn size(&self) -> Vec3<usize> {
        self.size
    }
}

impl Volume<bool> {
    pub fn from_sparse<I: Clone + Iterator<Item = Vec3<usize>>>(
        v: impl IntoIterator<Item = Vec3<usize>, IntoIter = I>,
    ) -> (Self, Vec3<usize>) {
        let iter = v.into_iter();
        let (min, max) = iter.clone().fold(
            (Vec3::broadcast(usize::MAX), Vec3::broadcast(usize::MIN)),
            |(min, max), p| (min.map2(p, |a, b| a.min(b)), max.map2(p, |a, b| a.max(b))),
        );
        let mut volume = Volume::new(max - min + 2);
        for p in iter {
            volume[p] = true;
        }

        (volume, min)
    }
}

impl<T> Index<Vec3<usize>> for Volume<T> {
    type Output = T;

    fn index(&self, index: Vec3<usize>) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl<T> IndexMut<Vec3<usize>> for Volume<T> {
    fn index_mut(&mut self, index: Vec3<usize>) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}
