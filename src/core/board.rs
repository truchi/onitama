use super::*;
use std::ops::Index;
use std::ops::IndexMut;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Board(pub [[Option<Piece>; SIZE]; SIZE]);

impl Default for Board {
    fn default() -> Self {
        macro_rules! file {
            ($piece:expr) => {
                [Some((Red, $piece)), None, None, None, Some((Blue, $piece))]
            };
        }

        Self([
            file!(Pawn(0)),
            file!(Pawn(1)),
            file!(King),
            file!(Pawn(2)),
            file!(Pawn(3)),
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
