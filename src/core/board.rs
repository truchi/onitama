use super::*;
use std::ops::Index;
use std::ops::IndexMut;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Board<T>(pub [[T; SIZE]; SIZE]);

impl<T> Index<Square> for Board<T> {
    type Output = T;

    fn index(&self, square: Square) -> &T {
        &self.0[square.file() as usize][square.rank() as usize]
    }
}

impl<T> IndexMut<Square> for Board<T> {
    fn index_mut(&mut self, square: Square) -> &mut T {
        &mut self.0[square.file() as usize][square.rank() as usize]
    }
}
