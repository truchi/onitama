use super::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Card {
    pub name:  &'static str,
    pub stamp: Player,
    pub moves: Moves,
}
