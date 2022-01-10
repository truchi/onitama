#![allow(unused)]

mod core;

pub use self::core::*;

fn main() {
    debug_cards();
    dbg!(std::mem::size_of::<Play>());
    dbg!(std::mem::size_of::<Move>());
    dbg!(std::mem::size_of::<Card>());
    dbg!(std::mem::size_of::<Square>());
    dbg!(std::mem::size_of::<Option<Square>>());
    dbg!(std::mem::size_of::<Side>());
    dbg!(std::mem::size_of::<Piece>());
    dbg!(std::mem::size_of::<Option<Piece>>());
    dbg!(std::mem::size_of::<Option<Piece>>() * SIZE * SIZE);
    dbg!(std::mem::size_of::<[Option<Piece>; SIZE * SIZE]>());
    dbg!(std::mem::size_of::<Board>());
    dbg!(std::mem::size_of::<Game>()); // 856...
    dbg!(std::mem::size_of::<P>());
    dbg!(std::mem::size_of::<Option<P>>());
    dbg!(std::mem::size_of::<Option<P>>() * SIZE * SIZE);
    dbg!(std::mem::size_of::<[Option<P>; SIZE * SIZE]>());

    enum P {
        King,
        PawnA,
        PawnB,
        PawnD,
        PawnE,
    }

    let board = Board::default();
    // dbg!(board[Square(A, One)]);
    // dbg!(board[Square(B, One)]);
    // dbg!(board[Square(C, One)]);
    // dbg!(board[Square(D, One)]);
    // dbg!(board[Square(E, One)]);
    // dbg!(board[Square(A, Five)]);
    // dbg!(board[Square(B, Five)]);
    // dbg!(board[Square(C, Five)]);
    // dbg!(board[Square(D, Five)]);
    // dbg!(board[Square(E, Five)]);
}
