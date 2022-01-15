use super::*;
use std::ops::Not;

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

    pub fn flipper(&self) -> fn(&Move) -> Move {
        if *self == Red {
            |mov| *mov
        } else {
            |mov| {
                let mut mov = *mov;
                mov.flip();
                mov
            }
        }
    }
}

impl Not for Player {
    type Output = Self;

    fn not(mut self) -> Self {
        self.invert();
        self
    }
}
