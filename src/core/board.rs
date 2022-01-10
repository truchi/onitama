use super::*;
use std::ops::Index;
use std::ops::IndexMut;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Board(pub [[Option<Piece>; SIZE]; SIZE]);

impl Default for Board {
    fn default() -> Self {
        macro_rules! file {
            ($red:expr, $blue:expr) => {
                [Some($red), None, None, None, Some($blue)]
            };
        }

        Self([
            file!(RedPawnA, BluePawnA),
            file!(RedPawnB, BluePawnB),
            file!(RedKing, BlueKing),
            file!(RedPawnD, BluePawnD),
            file!(RedPawnE, BluePawnE),
        ])
    }
}

impl Index<Square> for Board {
    type Output = Option<Piece>;

    fn index(&self, square: Square) -> &Option<Piece> {
        &self.0[square.file() as usize][square.rank() as usize]
    }
}

impl IndexMut<Square> for Board {
    fn index_mut(&mut self, square: Square) -> &mut Option<Piece> {
        &mut self.0[square.file() as usize][square.rank() as usize]
    }
}
