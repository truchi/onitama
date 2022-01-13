use super::*;

#[doc(hidden)]
macro_rules! declare {
    ($($Type:ident { $($U:literal $V:ident)* } $m:ident: $M:ty { $Neg:ident $Pos:ident })*) => { $(
        pub use $Type::*;

        #[derive(Copy, Clone, Eq, PartialEq, Debug)]
        pub enum $Type { $($V,)* }

        impl $Type {
            pub fn apply(&self, $m: $M) -> Option<Self> {
                let v = *self as usize;

                Self::try_from(match $m {
                    $Neg(u) => v.checked_sub(u)?,
                    $Pos(u) => v + u,
                })
                .ok()
            }
        }

        impl TryFrom<usize> for $Type {
            type Error = ();

            fn try_from(i: usize) -> Result<Self, ()> {
                match i {
                    $($U => Ok($V),)*
                    _ => Err(()),
                }
            }
        }
    )* };
}

declare!(
    File { 0 A   1 B   2 C     3 D    4 E    } horizontal: Horizontal   { Left Right }
    Rank { 0 One 1 Two 2 Three 3 Four 4 Five } vertical:   Vertical     { Down Up    }
);

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Square(pub File, pub Rank);

impl Square {
    pub fn file(&self) -> File {
        self.0
    }

    pub fn rank(&self) -> Rank {
        self.1
    }

    pub fn apply(&self, r#move: Move) -> Option<Self> {
        Some(Self(self.0.apply(r#move.1)?, self.1.apply(r#move.0)?))
    }

    pub fn king(player: Player) -> Self {
        Self(C, player.rank())
    }
}

impl From<(File, Rank)> for Square {
    fn from((file, rank): (File, Rank)) -> Self {
        Self(file, rank)
    }
}
