use std::ops::Deref;
use std::ops::DerefMut;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct List<T, const N: usize> {
    items: [T; N],
    len:   usize,
}

impl<T: Default, const N: usize> Default for List<T, N> {
    fn default() -> Self {
        Self {
            items: [(); N].map(|_| Default::default()),
            len:   0,
        }
    }
}

impl<T, const N: usize> AsRef<[T]> for List<T, N> {
    fn as_ref(&self) -> &[T] {
        self
    }
}

impl<T, const N: usize> AsMut<[T]> for List<T, N> {
    fn as_mut(&mut self) -> &mut [T] {
        self
    }
}

impl<T, const N: usize> Deref for List<T, N> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        unsafe { self.items.get_unchecked(..self.len) }
    }
}

impl<T, const N: usize> DerefMut for List<T, N> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe { self.items.get_unchecked_mut(..self.len) }
    }
}

impl<T, const N: usize> List<T, N> {
    pub fn swap_remove(&mut self, index: usize) -> Option<&T> {
        debug_assert!(index < self.len());

        self.len -= 1;

        if index == self.len {
            None
        } else {
            self.items.swap(index, self.len);
            self.get(index)
        }
    }
}
