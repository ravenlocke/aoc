use std::ops::Index;

#[derive(Clone, Copy)]
pub struct SmallVec<T, const M: usize> {
    data: [T; M],
    counter: usize,
}

impl<T: Default + Copy, const M: usize> Default for SmallVec<T, M> {
    fn default() -> Self {
        SmallVec {
            data: [T::default(); M],
            counter: 0,
        }
    }
}

impl<T: Copy, const M: usize> SmallVec<T, M> {
    pub fn push(&mut self, item: T) {
        self.data[self.counter] = item;
        self.counter += 1;
    }

    pub fn len(&self) -> usize {
        self.counter
    }

    pub fn pop(&mut self) -> T {
        self.counter -= 1;
        return self.data[self.counter];
    }
}

impl<'a, T, const M: usize> IntoIterator for &'a SmallVec<T, M> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data[0..self.counter].iter()
    }
}

impl<T, const M: usize> Index<usize> for SmallVec<T, M> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &self.data.get_unchecked(index) }
    }
}
