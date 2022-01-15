use super::*;

pub use Piece::*;
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Piece {
    King,
    PawnA,
    PawnB,
    PawnD,
    PawnE,
}

impl Piece {
    pub fn index(&self) -> usize {
        *self as usize
    }

    pub fn file(&self) -> File {
        match *self {
            PawnA => A,
            PawnB => B,
            King => C,
            PawnD => D,
            PawnE => E,
        }
    }
}

impl From<usize> for Piece {
    fn from(index: usize) -> Self {
        match index {
            0 => King,
            1 => PawnA,
            2 => PawnB,
            3 => PawnD,
            4 => PawnE,
            _ => panic!(),
        }
    }
}
