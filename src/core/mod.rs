mod board;
mod card;
mod game;
mod r#move;
mod piece;
mod player;
mod square;
mod utils;

pub use board::*;
pub use card::*;
pub use game::*;
pub use piece::*;
pub use player::*;
pub use r#move::*;
pub use square::*;

macro_rules! card {
    ($name:literal $stamp:ident [$($move:expr,)*]) => {
        Card { name: $name, stamp: $stamp, moves: &[ $( Move($move.0, $move.1), )* ] }
    };
}

pub const SIZE: usize = 5;
pub const HAND: usize = 2;
pub const CARDS: &[Card] = &[
    // ======= //
    // Neutral //
    // ======= //
    card!("Tiger" Blue [
        (Up(2), Right(0)),
        (Down(1), Right(0)),
    ]),
    card!("Crab" Blue [
        (Up(1), Right(0)),
        (Up(0), Right(2)),
        (Up(0), Left(2)),
    ]),
    card!("Monkey" Blue [
        (Up(1), Left(1)),
        (Up(1), Right(1)),
        (Down(1), Left(1)),
        (Down(1), Right(1)),
    ]),
    card!("Crane" Blue [
        (Up(1), Right(0)),
        (Down(1), Left(1)),
        (Down(1), Right(1)),
    ]),
    card!("Dragon" Red [
        (Up(1), Left(2)),
        (Up(1), Right(2)),
        (Down(1), Left(1)),
        (Down(1), Right(1)),
    ]),
    card!("Elephant" Red [
        (Up(1), Left(1)),
        (Up(1), Right(1)),
        (Up(0), Left(1)),
        (Up(0), Right(1)),
    ]),
    card!("Mantis" Red [
        (Up(1), Left(1)),
        (Up(1), Right(1)),
        (Down(1), Right(0)),
    ]),
    card!("Boar" Red [
        (Up(1), Right(0)),
        (Up(0), Left(1)),
        (Up(0), Right(1)),
    ]),
    // ==== //
    // Blue //
    // ==== //
    card!("Frog" Red [
        (Up(1), Left(1)),
        (Up(0), Left(2)),
        (Down(1), Right(1)),
    ]),
    card!("Goose" Blue [
        (Up(1), Left(1)),
        (Up(0), Left(1)),
        (Up(0), Right(1)),
        (Down(1), Right(1)),
    ]),
    card!("Horse" Red [
        (Up(1), Right(0)),
        (Up(0), Left(1)),
        (Down(1), Right(0)),
    ]),
    card!("Eel" Blue [
        (Up(1), Left(1)),
        (Up(0), Right(1)),
        (Down(1), Left(1)),
    ]),
    // ==== //
    // Red //
    // ==== //
    card!("Rabbit" Blue [
        (Up(1), Right(1)),
        (Up(0), Right(2)),
        (Down(1), Left(1)),
    ]),
    card!("Rooster" Red [
        (Up(1), Right(1)),
        (Up(0), Right(1)),
        (Up(0), Left(1)),
        (Down(1), Left(1)),
    ]),
    card!("Ox" Blue [
        (Up(1), Left(0)),
        (Up(0), Right(1)),
        (Down(1), Left(0)),
    ]),
    card!("Cobra" Red [
        (Up(1), Right(1)),
        (Up(0), Left(1)),
        (Down(1), Right(1)),
    ]),
];
