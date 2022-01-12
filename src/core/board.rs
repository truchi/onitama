use super::*;
use std::ops::Index;
use std::ops::IndexMut;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Board(pub [[Option<(Player, Piece)>; SIZE]; SIZE]);

impl Default for Board {
    fn default() -> Self {
        macro_rules! file {
            ($red:expr, $blue:expr) => {
                [Some((Red, $red)), None, None, None, Some((Blue, $blue))]
            };
        }

        Self([
            file!(PawnA, PawnA),
            file!(PawnB, PawnB),
            file!(King, King),
            file!(PawnD, PawnD),
            file!(PawnE, PawnE),
        ])
    }
}

impl Index<Square> for Board {
    type Output = Option<(Player, Piece)>;

    fn index(&self, square: Square) -> &Option<(Player, Piece)> {
        &self.0[square.file() as usize][square.rank() as usize]
    }
}

impl IndexMut<Square> for Board {
    fn index_mut(&mut self, square: Square) -> &mut Option<(Player, Piece)> {
        &mut self.0[square.file() as usize][square.rank() as usize]
    }
}
