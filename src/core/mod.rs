mod board;
mod card;
mod game;
mod r#move;
mod square;
mod utils;

pub use board::*;
pub use card::*;
pub use game::*;
pub use r#move::*;
pub use square::*;
pub use utils::*;

use std::ops::Not;

pub const SIZE: usize = 5;
pub const PAWNS: usize = SIZE - 1;
pub const HAND: usize = 2;
pub const PLAYS: usize = 20; // TODO ???

pub use Player::*;
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Player {
    Red,  // The "White" side (Rank::One)
    Blue, // The "Black" side (Rank::Five) (flipped)
}

impl Player {
    pub fn rank(&self) -> Rank {
        match *self {
            Red => One,
            Blue => Five,
        }
    }

    pub fn invert(&mut self) {
        *self = match *self {
            Red => Blue,
            Blue => Red,
        };
    }
}

impl Not for Player {
    type Output = Self;

    fn not(mut self) -> Self {
        self.invert();
        self
    }
}

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
