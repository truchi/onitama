#![allow(unused)]

mod core;

pub use self::core::*;

fn main() {
    let board = Board([
        [Some((Red, King)), None, None, None, None],
        [None, None, None, None, None],
        [None, None, None, None, None],
        [None, None, None, None, None],
        [None, None, None, None, None],
    ]);

    // dbg!(board[Square(A, One)]);
    // dbg!(CARDS);

    debug_cards();
}
