#![allow(dead_code)]

use std::{
    cmp::Ordering,
    collections::hash_map::DefaultHasher,
    fmt,
    hash::{Hash, Hasher},
    ops::{self, Index, IndexMut}, marker::PhantomData,
};

use itertools::Itertools;
use num_traits::{AsPrimitive, Num, PrimInt, Signed, Unsigned, Zero};

#[derive(PartialEq, Eq)]
pub enum WrapMode {
    Clamped,
    Wrapped,
}

pub trait Collider<T> {
    fn collides(&self, cell: &T) -> bool;
}

impl<T> Collider<T> for () {
    fn collides(&self, cell: &T) -> bool {
        false
    }
}

impl<T, F: for<'a> Fn(&'a T) -> bool> Collider<T> for F {
    fn collides(&self, cell: &T) -> bool {
        self(cell)
    }
}

pub struct GridWalker<'a, T, C: Collider<T> = ()> {
    mode: WrapMode,
    collider: Option<C>,
    grid: &'a Grid<T>,
    x: usize,
    y: usize,
}

impl<'a, T, C: Collider<T>> GridWalker<'a, T, C> {
    pub fn clamped(grid: &'a Grid<T>) -> Self {
        Self {
            mode: WrapMode::Clamped,
            grid,
            x: 0,
            y: 0,
            collider: None,
        }
    }
    pub fn wrapped(grid: &'a Grid<T>) -> Self {
        Self {
            mode: WrapMode::Wrapped,
            grid,
            x: 0,
            y: 0,
            collider: None,
        }
    }
    pub fn collider(&mut self, collider: C) -> &mut Self {
        self.collider = Some(collider);
        self
    }
    pub fn collide_off(&mut self) -> &mut Self {
        self.collider = None;
        self
    }
    pub fn mode(&mut self, mode: WrapMode) -> &mut Self {
        self.mode = mode;
        self
    }

    pub fn up(&mut self) {
        self.try_offset(0, -1);
    }
    pub fn down(&mut self) {
        self.try_offset(0, 1);
    }
    pub fn left(&mut self) {
        self.try_offset(-1, 0);
    }
    pub fn right(&mut self) {
        self.try_offset(1, 0);
    }

    fn apply_offset(&self, x: i32, y: i32) -> (usize, usize) {
        let x = self.x as i64 + x as i64;
        let y = self.y as i64 + y as i64;
        match self.mode {
            WrapMode::Clamped => (
                x.clamp(0, (self.grid.width - 1) as i64) as usize,
                y.clamp(0, (self.grid.height - 1) as i64) as usize,
            ),
            WrapMode::Wrapped => (
                (x % self.grid.width as i64 + self.grid.width as i64) as usize % self.grid.width,
                (y % self.grid.height as i64 + self.grid.height as i64) as usize % self.grid.height,
            ),
        }
    }

    pub fn try_offset(&mut self, x: i32, y: i32) {
        let (x, y) = self.apply_offset(x, y);
        if !self.collides(&self.grid[(x, y)]) {
            self.x = x;
            self.y = y;
        }
    }
    pub fn tp(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }

    fn collides(&self, t: &T) -> bool {
        self.collider.as_ref().map_or(false, |collider| collider.collides(t))
    }

    pub fn pos(&self) -> (usize, usize) {
        (self.x, self.y)
    }
    pub fn peek(&self, x: i32, y: i32) -> &T {
        let (x, y) = self.apply_offset(x, y);
        &self.grid[(x, y)]
    }
    pub fn get(&self) -> &T {
        &self.grid[(self.x, self.y)]
    }
}

pub struct GridEnumerator<'a, T> {
    grid: &'a Grid<T>,
    x: usize,
    y: usize,
}

impl<'a, T> Iterator for GridEnumerator<'a, T> {
    type Item = (&'a T, usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.grid.get(self.x, self.y).map(|t| (t, self.x, self.y));
        self.x += 1;
        if self.x >= self.grid.width {
            self.x = 0;
            self.y += 1;
        }
        ret
    }
}

pub struct Rect<T> {
    x: T,
    y: T,
    width: usize,
    height: usize,
}

impl<T> Rect<T> {
    pub fn new(x: T, y: T, width: usize, height: usize) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
    pub fn center(x: T, y: T, extent: usize) -> Self
    where
        T: ops::Sub<T, Output = T> + TryFrom<usize> + Default,
    {
        Self {
            x: x - T::try_from(extent).unwrap_or_default(),
            y: y - T::try_from(extent).unwrap_or_default(),
            width: extent * 2 + 1,
            height: extent * 2 + 1,
        }
    }
}

pub struct GridSlice<'a, T, I: AsPrimitive<usize> + ops::Add<I, Output = I>>
where
    usize: AsPrimitive<I>,
{
    grid: &'a Grid<T>,
    rect: Rect<I>,
}

pub struct GridSliceMut<'a, T, I: AsPrimitive<usize> + ops::Add<I, Output = I>> {
    grid: &'a mut Grid<T>,
    rect: Rect<I>,
}

impl<'a, T, I: AsPrimitive<usize> + ops::Add<I, Output = I>> GridSlice<'a, T, I>
where
    usize: AsPrimitive<I>,
{
    pub fn iter(&self) -> GridSliceIter<T, I>
    where
        I: Default,
    {
        GridSliceIter {
            grid_slice: self,
            x: 0,
            y: 0,
        }
    }
}

impl<'a, T, I: AsPrimitive<usize> + ops::Add<I, Output = I> + std::cmp::PartialOrd> GridIndex
    for GridSlice<'a, T, I>
where
    usize: AsPrimitive<I>,
{
    type Output = T;

    fn get_start(&self) -> (usize, usize) {
        (0, 0)
    }
    fn get_size(&self) -> (usize, usize) {
        (self.rect.width, self.rect.height)
    }
    fn get_unchecked(&self, x: usize, y: usize) -> &Self::Output {
        self.grid
            .get_unchecked((self.rect.x + x.as_()).as_(), (self.rect.y + y.as_()).as_())
    }

    fn extra_check(&self, x: usize, y: usize) -> bool {
        self.rect.x + x.as_() >= 0.as_()
            && self.rect.y + y.as_() >= 0.as_()
            && self.rect.x + x.as_() < self.grid.width.as_()
            && self.rect.y + y.as_() < self.grid.height.as_()
    }

    unsafe fn get_unchecked_unsafe(&self, x: usize, y: usize) -> &Self::Output {
        self.grid.get_unchecked_unsafe((self.rect.x + x.as_()).as_(), (self.rect.y + y.as_()).as_())
    }
}

impl<'a, T, I: AsPrimitive<usize> + ops::Add<I, Output = I> + std::cmp::PartialOrd> GridIndex
    for GridSliceMut<'a, T, I>
where
    usize: AsPrimitive<I>,
{
    type Output = T;

    fn get_start(&self) -> (usize, usize) {
        (0, 0)
    }
    fn get_size(&self) -> (usize, usize) {
        (self.rect.width, self.rect.height)
    }
    fn get_unchecked(&self, x: usize, y: usize) -> &Self::Output {
        self.grid
            .get_unchecked((self.rect.x + x.as_()).as_(), (self.rect.y + y.as_()).as_())
    }

    fn extra_check(&self, x: usize, y: usize) -> bool {
        self.rect.x + x.as_() >= 0.as_()
            && self.rect.y + y.as_() >= 0.as_()
            && self.rect.x + x.as_() < self.grid.width.as_()
            && self.rect.y + y.as_() < self.grid.height.as_()
    }
    
    unsafe fn get_unchecked_unsafe(&self, x: usize, y: usize) -> &Self::Output {
        self.grid.get_unchecked_unsafe((self.rect.x + x.as_()).as_(), (self.rect.y + y.as_()).as_())
    }
}

impl<'a, T, I: AsPrimitive<usize> + ops::Add<I, Output = I> + std::cmp::PartialOrd> GridIndexMut
    for GridSliceMut<'a, T, I>
where
    usize: AsPrimitive<I>,
{
    fn get_mut_unchecked(&mut self, x: usize, y: usize) -> &mut Self::Output {
        self.grid
            .get_mut_unchecked((self.rect.x + x.as_()).as_(), (self.rect.y + y.as_()).as_())
    }
    unsafe fn get_mut_unchecked_unsafe(&mut self, x: usize, y: usize) -> &mut Self::Output {
        self.grid.get_mut_unchecked_unsafe((self.rect.x + x.as_()).as_(), (self.rect.y + y.as_()).as_())
    }
}

pub struct GridSliceIter<'a, T, I: AsPrimitive<usize> + ops::Add<I, Output = I>>
where
    usize: AsPrimitive<I>,
{
    grid_slice: &'a GridSlice<'a, T, I>,
    x: usize,
    y: usize,
}
impl<'a, T, I: AsPrimitive<usize> + ops::Add<I, Output = I> + std::cmp::PartialOrd> Iterator
    for GridSliceIter<'a, T, I>
where
    usize: AsPrimitive<I>,
{
    type Item = Option<&'a T>;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = if self.y >= self.grid_slice.rect.height {
            None
        } else {
            Some(self.grid_slice.get(self.x, self.y))
        };
        self.x += 1;
        if self.x >= self.grid_slice.rect.width {
            self.x = 0;
            self.y += 1;
        }
        ret
    }
}

pub trait CollectGridFlat<T> {
    fn collect_grid_f(self, width: usize, height: usize) -> Grid<T>;
    fn collect_rows(self, width: usize) -> Grid<T>;
    fn collect_columns(self, height: usize) -> Grid<T>;
}

impl<I: Iterator> CollectGridFlat<I::Item> for I {
    // Panics if the length of the iterator isn't equal to width * height
    fn collect_grid_f(self, width: usize, height: usize) -> Grid<I::Item> {
        let data = self.collect_vec();
        assert_eq!(data.len(), width * height);
        Grid {
            data,
            width,
            height,
        }
    }
    // Panics if not all elements fit into row
    fn collect_rows(self, width: usize) -> Grid<I::Item> {
        let data = self.collect_vec();
        assert_eq!(data.len() % width, 0);
        let height = data.len() / width;
        Grid {
            data,
            width,
            height,
        }
    }

    // Panics if not all elements fit into column
    fn collect_columns(self, height: usize) -> Grid<I::Item> {
        let data = self.collect_vec();
        assert_eq!(data.len() % height, 0);
        let width = data.len() / height;
        Grid {
            data,
            width,
            height,
        }
    }
}

pub trait CollectGrid<T> {
    fn collect_grid(self) -> Grid<T>;
}

impl<I: Iterator> CollectGrid<<I::Item as Iterator>::Item> for I
where
    I::Item: Iterator,
    <I::Item as Iterator>::Item: Clone,
{
    fn collect_grid(self) -> Grid<<I::Item as IntoIterator>::Item> {
        let mut data = Vec::new();
        let mut true_width = 0;
        let mut width = 0;
        let mut height = 0;
        self.for_each(|row| {
            row.for_each(|tile| {
                width += 1;
                data.push(tile);
            });
            if true_width == 0 {
                true_width = width;
            } else if true_width != width {
                panic!("Row width varies");
            }
            height += 1;
        });
        Grid {
            data,
            width: true_width,
            height,
        }
    }
}

pub trait IntoGridFlat<T> {
    fn into_grid_f(self, width: usize, height: usize) -> Grid<T>;
}

impl<T> IntoGridFlat<T> for Vec<T> {
    fn into_grid_f(self, width: usize, height: usize) -> Grid<T> {
        assert_eq!(self.len(), width * height);
        Grid {
            data: self,
            width,
            height,
        }
    }
}

impl<T, const L: usize> IntoGridFlat<T> for [T; L] {
    fn into_grid_f(self, width: usize, height: usize) -> Grid<T> {
        assert_eq!(self.len(), width * height);
        let mut data = Vec::with_capacity(self.len());
        for item in self {
            data.push(item);
        }
        Grid {
            data,
            width,
            height,
        }
    }
}

pub trait GridLinearSlice<'a>: Sized {
    type Output;
    fn len(&self) -> usize;
    fn get(&self, i: usize) -> Option<&Self::Output>;

    fn iter(self) -> GridLinearIter<'a, Self> {
        GridLinearIter {
            front: 0,
            back: self.len() as isize - 1,
            slice: self,
            _marker: PhantomData,
        }
    }

    fn from_grid(grid: &'a Grid<Self::Output>, i: usize) -> Option<Self>;
}

pub trait GridLinearSliceMut<'a>: GridLinearSlice<'a> {
    fn get_mut(&mut self, i: usize) -> Option<&mut Self::Output>;

    fn iter_mut(self) -> GridLinearIterMut<'a, Self> {
        GridLinearIterMut {
            front: 0,
            back: self.len() as isize - 1,
            slice: self,
            _marker: PhantomData,
        }
    }

    fn from_grid_mut(grid: &'a mut Grid<Self::Output>, i: usize) -> Option<Self>;
}

pub struct GridLinearIter<'a, L: GridLinearSlice<'a> + 'a> {
    slice: L,
    front: isize,
    back: isize,
    _marker: PhantomData<&'a ()>,
}

impl<'a, L: GridLinearSlice<'a>> Iterator for GridLinearIter<'a, L> {
    type Item = &'a L::Output;

    fn next(&mut self) -> Option<Self::Item> {
        if self.front <= self.back {
            let ret = self.slice.get(self.front.try_into().ok()?);
            self.front += 1;
            ret.map(|r| unsafe { &*( r as *const _) })
        } else {
            None
        }
    }
}

impl<'a, L: GridLinearSlice<'a>> DoubleEndedIterator for GridLinearIter<'a, L> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.front <= self.back {
            let ret = self.slice.get(self.back.try_into().ok()?);
            self.back -= 1;
            ret.map(|r| unsafe { &*( r as *const _) })
        } else {
            None
        }
    }
}

pub struct GridLinearIterMut<'a, L: GridLinearSlice<'a> + 'a> {
    slice: L,
    front: isize,
    back: isize,
    _marker: PhantomData<&'a mut ()>,
}

impl<'a, L: GridLinearSliceMut<'a>> Iterator for GridLinearIterMut<'a, L> {
    type Item = &'a mut L::Output;

    fn next(&mut self) -> Option<Self::Item> {
        if self.front <= self.back {
            let ret = self.slice.get_mut(self.front.try_into().ok()?);
            self.front += 1;
            ret.map(|r| unsafe { &mut *( r as *mut _) })
        } else {
            None
        }
    }
}

impl<'a, L: GridLinearSliceMut<'a>> DoubleEndedIterator for GridLinearIterMut<'a, L> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.front <= self.back {
            let ret = self.slice.get_mut(self.back.try_into().ok()?);
            self.back -= 1;
            ret.map(|r| unsafe { &mut *( r as *mut _) })
        } else {
            None
        }
    }
}

pub struct GridLinear<'a, S: GridLinearSlice<'a>> {
    grid: &'a Grid<S::Output>,
    front: isize,
    back: isize,
}

impl<'a, S: GridLinearSlice<'a>> Iterator for GridLinear<'a, S> {
    type Item = S;

    fn next(&mut self) -> Option<Self::Item> {
        if self.front <= self.back {
            let l = S::from_grid(self.grid, self.front.try_into().ok()?);
            self.front += 1;
            l
        } else {
            None
        }
    }
}

impl<'a, S: GridLinearSlice<'a>> DoubleEndedIterator for GridLinear<'a, S> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.front <= self.back {
            let l = S::from_grid(self.grid, self.back.try_into().ok()?);
            self.back -= 1;
            l
        } else {
            None
        }
    }
}

pub struct GridLinearMut<'a, S: GridLinearSliceMut<'a>> {
    grid: &'a mut Grid<S::Output>,
    front: isize,
    back: isize,
}

impl<'a, S: GridLinearSliceMut<'a>> Iterator for GridLinearMut<'a, S> {
    type Item = S;

    fn next(&mut self) -> Option<Self::Item> {
        if self.front <= self.back {
            let l = S::from_grid_mut(unsafe { &mut *(self.grid as *mut _) }, self.front.try_into().ok()?);
            self.front += 1;
            l
        } else {
            None
        }
    }
}

impl<'a, S: GridLinearSliceMut<'a>> DoubleEndedIterator for GridLinearMut<'a, S> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.front <= self.back {
            let l = S::from_grid_mut(unsafe { &mut *(self.grid as *mut _) }, self.back.try_into().ok()?);
            self.back -= 1;
            l
        } else {
            None
        }
    }
}

#[derive(Clone, Copy)]
pub struct GridRowSlice<'a, T> {
    grid: &'a Grid<T>,
    y: usize,
}

impl<'a, T> Index<usize> for GridRowSlice<'a, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl<'a, T> GridLinearSlice<'a> for GridRowSlice<'a, T> {
    type Output = T;
    fn len(&self) -> usize {
        self.grid.width
    }

    fn get(&self, i: usize) -> Option<&T> {
        self.grid.get(i, self.y)
    }

    fn from_grid(grid: &'a Grid<Self::Output>, i: usize) -> Option<Self> {
        grid.get_row(i)
    }
}

pub struct GridRowSliceMut<'a, T> {
    grid: &'a mut Grid<T>,
    y: usize,
}

impl<'a, T> Index<usize> for GridRowSliceMut<'a, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl<'a, T> IndexMut<usize> for GridRowSliceMut<'a, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}

impl<'a, T> GridLinearSlice<'a> for GridRowSliceMut<'a, T> {
    type Output = T;
    fn len(&self) -> usize {
        self.grid.width
    }

    fn get(&self, i: usize) -> Option<&T> {
        self.grid.get(i, self.y)
    }

    fn from_grid(grid: &Grid<Self::Output>, i: usize) -> Option<Self> {
        panic!()
    }
}

impl<'a, T> GridLinearSliceMut<'a> for GridRowSliceMut<'a, T> {
    fn get_mut(&mut self, i: usize) -> Option<&mut T> {
        self.grid.get_mut(i, self.y)
    }

    fn from_grid_mut(grid: &'a mut Grid<Self::Output>, i: usize) -> Option<Self> {
        grid.get_row_mut(i)
    }
}

#[derive(Clone, Copy)]
pub struct GridColumnSlice<'a, T> {
    grid: &'a Grid<T>,
    x: usize,
}

impl<'a, T> Index<usize> for GridColumnSlice<'a, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl<'a, T> GridLinearSlice<'a> for GridColumnSlice<'a, T> {
    type Output = T;
    fn len(&self) -> usize {
        self.grid.height
    }

    fn get(&self, i: usize) -> Option<&T> {
        self.grid.get(self.x, i)
    }

    fn from_grid(grid: &'a Grid<Self::Output>, i: usize) -> Option<Self> {
        grid.get_column(i)
    }
}

pub struct GridColumnSliceMut<'a, T> {
    grid: &'a mut Grid<T>,
    x: usize,
}

impl<'a, T> Index<usize> for GridColumnSliceMut<'a, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl<'a, T> IndexMut<usize> for GridColumnSliceMut<'a, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}

impl<'a, T> GridLinearSlice<'a> for GridColumnSliceMut<'a, T> {
    type Output = T;
    fn len(&self) -> usize {
        self.grid.height
    }

    fn get(&self, i: usize) -> Option<&T> {
        self.grid.get(self.x, i)
    }

    fn from_grid(grid: &'a Grid<Self::Output>, i: usize) -> Option<Self> {
        panic!()
    }
}

impl<'a, T> GridLinearSliceMut<'a> for GridColumnSliceMut<'a, T> {
    fn get_mut(&mut self, i: usize) -> Option<&mut T> {
        self.grid.get_mut(self.x, i)
    }

    fn from_grid_mut(grid: &'a mut Grid<Self::Output>, i: usize) -> Option<Self> {
        grid.get_column_mut(i)
    }
}

pub trait IntoGrid<T> {
    fn into_grid(self) -> Grid<T>;
}

impl<T> IntoGrid<T> for Vec<Vec<T>> {
    fn into_grid(self) -> Grid<T> {
        if self.is_empty() || self[0].is_empty() {
            return Grid::empty();
        }
        let height = self.len();
        let width = self[0].len();
        let mut data = Vec::with_capacity(height * width);
        for row in self {
            assert_eq!(row.len(), width);
            for tile in row {
                data.push(tile);
            }
        }
        Grid {
            data,
            width,
            height,
        }
    }
}

impl<T, const H: usize, const W: usize> IntoGrid<T> for [[T; W]; H] {
    fn into_grid(self) -> Grid<T> {
        let mut data = Vec::with_capacity(W * H);
        for row in self {
            for tile in row {
                data.push(tile);
            }
        }
        Grid {
            data,
            width: W,
            height: H,
        }
    }
}

pub struct Grid<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Grid<T> {
    pub fn new(width: usize, height: usize) -> Self
    where
        T: Default + Clone,
    {
        Self {
            data: vec![T::default(); height * width],
            width,
            height,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn empty() -> Self {
        Self {
            data: Vec::new(),
            width: 0,
            height: 0,
        }
    }

    pub fn from_input<F: FnMut(char) -> Option<T>>(input: &str, mut map: F) -> Self
    where
        T: Default,
    {
        let mut data = Vec::new();
        let mut true_width = 0;
        let mut width = 0;
        let mut height = 0;
        for char in input.chars() {
            if char != '\n' {
                if let Some(v) = map(char) {
                    width += 1;
                    data.push(v);
                }
            } else {
                if true_width == 0 {
                    true_width = width;
                } else if width != true_width {
                    for _ in width..true_width {
                        data.push(T::default())
                    }
                }
                height += 1;
            }
        }
        if width != 0 {
            if true_width == 0 {
                true_width = width;
            }
            if width != true_width {
                for _ in width..true_width {
                    data.push(T::default())
                }
            }
            height += 1;
        }

        Self {
            data,
            width: true_width,
            height,
        }
    }

    pub fn to_input<F: FnMut(&T) -> char>(&self, mut map: F) -> String {
        let mut string = String::with_capacity((self.width + 1) * self.height);
        for row in self.rows() {
            for cell in row.iter() {
                string.push(map(cell));
            }
            string.push('\n');
        }
        string
    } 

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.data.iter()
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
        self.data.iter_mut()
    }
    pub fn enumerate(&self) -> GridEnumerator<T> {
        GridEnumerator {
            grid: self,
            x: 0,
            y: 0,
        }
    }

    pub fn get_slice<I: AsPrimitive<usize> + ops::Add<I, Output = I>>(
        &self,
        rect: Rect<I>,
    ) -> GridSlice<T, I>
    where
        usize: AsPrimitive<I>,
    {
        GridSlice { grid: self, rect }
    }

    pub fn rows(&self) -> GridLinear<GridRowSlice<T>> {
        GridLinear {
            grid: self,
            front: 0,
            back: self.height as isize - 1,
        }
    }
    
    pub fn rows_mut(&mut self) -> GridLinearMut<GridRowSliceMut<T>> {
        GridLinearMut {
            front: 0,
            back: self.height as isize - 1,
            grid: self,
        }
    }

    pub fn columns(&self) -> GridLinear<GridColumnSlice<T>> {
        GridLinear {
            grid: self,
            front: 0,
            back: self.width as isize - 1,
        }
    }
    
    pub fn columns_mut(&mut self) -> GridLinearMut<GridColumnSliceMut<T>> {
        GridLinearMut {
            front: 0,
            back: self.width as isize - 1,
            grid: self,
        }
    }

    pub fn get_row(&self, y: usize) -> Option<GridRowSlice<T>> {
        if y < self.height {
            Some(GridRowSlice { grid: self, y })
        } else {
            None
        }
    }

    pub fn get_row_mut(&mut self, y: usize) -> Option<GridRowSliceMut<T>> {
        if y < self.height {
            Some(GridRowSliceMut { grid: self, y })
        } else {
            None
        }
    }

    pub fn get_column(&self, x: usize) -> Option<GridColumnSlice<T>> {
        if x < self.width {
            Some(GridColumnSlice { grid: self, x })
        } else {
            None
        }
    }

    pub fn get_column_mut(&mut self, x: usize) -> Option<GridColumnSliceMut<T>> {
        if x < self.width {
            Some(GridColumnSliceMut { grid: self, x })
        } else {
            None
        }
    }

    pub fn get_slice_mut<I: AsPrimitive<usize> + ops::Add<I, Output = I>>(
        &mut self,
        rect: Rect<I>,
    ) -> GridSliceMut<T, I>
    where
        usize: AsPrimitive<I>,
    {
        GridSliceMut { grid: self, rect }
    }

    pub fn get_size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn indices(&self) -> GridIndexIter {
        GridIndexIter {
            x: 0,
            y: 0,
            width: self.width,
            height: self.height,
        }
    }

    pub fn find_path<N: Neighbors>(
        &self,
        start: (usize, usize),
        end: (usize, usize),
        mut transition: impl FnMut(&T, (usize, usize), &T, (usize, usize)) -> Option<f64>,
    ) -> Option<(Vec<(usize, usize)>, f64)> {
        pathfinding::prelude::astar(
            &start,
            |node| {
                let from = self.get(node.0, node.1).unwrap();
                let mut neighbors = Vec::new();

                for n in N::NEIGHBORS {
                    let res = node
                        .0
                        .checked_add_signed(n.0)
                        .zip(node.1.checked_add_signed(n.1))
                        .and_then(|n| {
                            self.get(n.0, n.1)
                                .and_then(|to| transition(from, *node, to, n).map(|t| (t, n)))
                        });
                    if let Some((transition, n)) = res {
                        neighbors.push((n, super::OrderedFloat(transition)));
                    }
                }

                neighbors
            },
            |node| {
                let x = node.0.abs_diff(end.0);
                let y = node.1.abs_diff(end.1);
                super::OrderedFloat(((x * x + y * y) as f64).sqrt())
            },
            |node| *node == end,
        )
        .map(|(path, cost)| (path, cost.0))
    }
}

pub trait Neighbors {
    const NEIGHBORS: &'static [(isize, isize)];
}

pub struct Cardinals;

impl Neighbors for Cardinals {
    const NEIGHBORS: &'static [(isize, isize)] = &[(1, 0), (0, 1), (-1, 0), (0, -1)];
}

struct Close;

impl Neighbors for Close {
    const NEIGHBORS: &'static [(isize, isize)] = &[
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (1, -1),
    ];
}

impl<T: fmt::Display> fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let displayed: Vec<String> = self.data.iter().map(|v| v.to_string()).collect();
        let len = displayed.iter().map(|s| s.chars().count()).max().unwrap_or(0);
        for (i, s) in displayed.into_iter().enumerate() {
            for _ in 0..len - s.len() {
                write!(f, " ")?;
            }
            write!(f, "{}", s)?;
            if i % self.width == self.width - 1 {
                writeln!(f)?;
            } else {
                write!(f, " ")?;
            }
        }
        Ok(())
    }
}

pub trait GridIndex {
    type Output;
    fn get_start(&self) -> (usize, usize);
    fn get_size(&self) -> (usize, usize);
    fn get_unchecked(&self, x: usize, y: usize) -> &Self::Output;
    unsafe fn get_unchecked_unsafe(&self, x: usize, y: usize) -> &Self::Output;
    fn extra_check(&self, x: usize, y: usize) -> bool;

    fn get<I>(&self, x: I, y: I) -> Option<&Self::Output>
    where
        I: PrimInt + TryInto<usize>,
    {
        let (width, height) = self.get_size();
        let (x, y) = (x.try_into().ok()?, y.try_into().ok()?);
        if x < width && y < height && self.extra_check(x, y) {
            Some(self.get_unchecked(x, y))
        } else {
            None
        }
    }

    fn transposed(&self) -> Grid<Self::Output>
    where
        Self::Output: Clone,
    {
        let (width, height) = self.get_size();
        let mut data = Vec::with_capacity(width * height);

        for x in 0..width {
            for y in 0..height {
                data.push(self.get_unchecked(x, y).clone())
            }
        }

        Grid {
            data,
            width: height,
            height: width,
        }
    }
}

pub trait GridIndexMut: GridIndex {
    fn get_mut_unchecked(&mut self, x: usize, y: usize) -> &mut Self::Output;
    unsafe fn get_mut_unchecked_unsafe(&mut self, x: usize, y: usize) -> &mut Self::Output;
    fn get_mut<I>(&mut self, x: I, y: I) -> Option<&mut Self::Output>
    where
        I: PrimInt + TryInto<usize>,
    {
        let (width, height) = self.get_size();
        let (x, y) = (x.try_into().ok()?, y.try_into().ok()?);
        if x < width && y < height {
            Some(self.get_mut_unchecked(x, y))
        } else {
            None
        }
    }
    fn set_all(&mut self, value: Self::Output)
    where
        Self::Output: Clone,
    {
        let (width, height) = self.get_size();
        for y in 0..height {
            for x in 0..width {
                self.get_mut_unchecked(x, y).clone_from(&value);
            }
        }
    }
}

impl<T> GridIndex for Grid<T> {
    type Output = T;
    fn get_start(&self) -> (usize, usize) {
        (0, 0)
    }
    fn get_size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    fn get_unchecked(&self, x: usize, y: usize) -> &T {
        &self.data[y * self.width + x]
    }

    fn extra_check(&self, x: usize, y: usize) -> bool {
        true
    }

    unsafe fn get_unchecked_unsafe(&self, x: usize, y: usize) -> &Self::Output {
        self.data.get_unchecked(y * self.width + x)
    }
}

impl<T> GridIndexMut for Grid<T> {
    fn get_mut_unchecked(&mut self, x: usize, y: usize) -> &mut Self::Output {
        &mut self.data[y * self.width + x]
    }

    unsafe fn get_mut_unchecked_unsafe(&mut self, x: usize, y: usize) -> &mut Self::Output {
        self.data.get_unchecked_mut(y * self.width + x)
    }
}

impl Grid<bool> {
    pub fn from_map(input: &str) -> Self {
        Self::from_input(input, |c| match c {
            '#' | '1' => Some(true),
            '.' | '0' => Some(false),
            _ => None,
        })
    }

    pub fn to_map(&self) -> String {
        let mut output = String::with_capacity((self.width + 1) * self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                output.push(if *self.get_unchecked(x, y) { '#' } else { '.' });
            }
            output.push('\n');
        }
        output
    }

    pub fn parse_word(&self) -> String {
        assert_eq!(self.height, 6, "Can only parse character of height 6");
        assert_eq!(
            self.width % 5,
            0,
            "Can only parse characters of width 4, with a space of 1"
        );
        let l = self.width / 5;
        let mut result = String::with_capacity(l);
        // println!("{}", self.to_map());
        for i in 0..l {
            let slice = self.get_slice(Rect::new(i * 5, 0, 4, 6));

            let w = slice
                .iter()
                .map(|b| {
                    b.map(|b| *b as u32).expect("This shouldn't happen since we've asserted the size at the top of the function.")
                }).enumerate().fold(0, |acc, (i, v)| acc | (v << i));

            // println!("{}", w);

            result.push(match w {
                10090902 => 'A',
                7968663 => 'B',
                15800095 => 'E',
                1120031 => 'F',
                15323542 => 'G',
                10067865 => 'H',
                6916236 => 'J',
                15798545 => 'L',
                6920598 => 'O',
                1145239 => 'P',
                9795991 => 'R',
                7889182 => 'S',
                6920601 => 'U',
                15803535 => 'Z',
                0 => ' ',
                _ => '?',
            })
        }
        result
    }

    pub fn collides(&self, other: &Grid<bool>, offset: (usize, usize)) -> bool {
        if offset.0 + other.width > self.width || offset.1 + other.height > self.height {
            return true;
        }
        for y in 0..other.height {
            for x in 0..other.width {
                if self[(x + offset.0, y + offset.1)] && other[(x, y)] {
                    return true;
                }
            }
        }
        false
    }
    pub fn or(&mut self, other: &Grid<bool>, offset: (usize, usize)) {
        if offset.0 < self.width && offset.1 < self.height {
            for y in 0..other.height.min(self.height - offset.1) {
                for x in 0..other.width.min(self.width - offset.0) {
                    let other = other[(x, y)];
                    self[(x + offset.0, y + offset.1)] |= other;
                }
            }
        }
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        self.get(index.0, index.1).unwrap()
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        self.get_mut(index.0, index.1).unwrap()
    }
}

impl<T: Clone> Clone for Grid<T> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            width: self.width,
            height: self.height,
        }
    }
}

pub struct GridIndexIter {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

impl Iterator for GridIndexIter {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.y < self.height {
            let result = Some((self.x, self.y));
            self.x += 1;
            self.y += self.x / self.width;
            self.x %= self.width;
            result
        } else {
            None
        }
    }
}
