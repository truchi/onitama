use super::*;

#[doc(hidden)]
macro_rules! declare {
    ($($Type:ident { $Neg:ident $Pos:ident })*) => { $(
        pub use $Type::*;

        #[derive(Copy, Clone, Eq, PartialEq, Debug)]
        pub enum $Type {
            $Neg(usize),
            $Pos(usize),
        }

        impl $Type {
            pub fn flip(&mut self) {
                *self = match *self {
                    $Neg(u) => $Pos(u),
                    $Pos(u) => $Neg(u),
                }
            }
        }
    )* };
}

declare!(
    Vertical   { Down Up    }
    Horizontal { Left Right }
);

pub type Moves = &'static [Move];

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Move(pub Vertical, pub Horizontal);

impl Move {
    pub fn vertical(&self) -> Vertical {
        self.0
    }

    pub fn horizontal(&self) -> Horizontal {
        self.1
    }

    pub fn flip(&mut self) {
        self.0.flip();
        self.1.flip();
    }

    pub fn flip_for(&mut self, player: Player) {
        if player == Blue {
            self.flip();
        }
    }
}
