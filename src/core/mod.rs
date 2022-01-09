mod board;
mod card;
mod game;
mod r#move;
mod square;

pub use board::*;
pub use card::*;
pub use game::*;
pub use r#move::*;
pub use square::*;

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

pub use PieceType::*;
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum PieceType {
    King,
    Pawn(usize),
}

impl PieceType {
    pub fn pawn_mut(&mut self) -> Option<&mut usize> {
        match self {
            King => None,
            Pawn(pawn) => Some(pawn),
        }
    }
}

pub type Piece = (Player, PieceType);
