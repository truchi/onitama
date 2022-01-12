use super::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Card {
    pub name:  &'static str,
    pub stamp: Player,
    pub moves: Moves,
}

macro_rules! card {
    ($name:literal $stamp:ident [$($move:expr,)*]) => {
        Card { name: $name, stamp: $stamp, moves: &[ $(
            Move($move.0, $move.1),
        )* ] }
    };
}

// see https://i.pinimg.com/originals/3b/13/0a/3b130a637e6fa05ec4c74d00efbc67b9.jpg
// see https://www.gadgetsville.store/wp-content/uploads/2017/12/16096-c.jpg
// see https://mothmeeple.files.wordpress.com/2019/02/20190113_162110-1.jpg?w=663&h=197

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

pub fn debug_cards() {
    for &card in CARDS {
        debug_card(card);
    }
}

pub fn debug_card(card: Card) {
    println!("{}", card.name);

    let center = Square(C, Three);
    let mut cells = [[false; SIZE]; SIZE];

    for &mov in card.moves {
        let square = center.apply(mov).unwrap();
        let file = (SIZE - 1) - square.file() as usize;
        let rank = square.rank() as usize;

        cells[file][rank] = true;
    }

    assert!(cells[2][2] == false);

    println!("╭─────╮");
    for (i, row) in cells.into_iter().enumerate() {
        print!("│");
        for (j, col) in row.into_iter().enumerate() {
            if col {
                print!("\x1B[31m*\x1B[0m");
            } else {
                if i == 2 && j == 2 {
                    print!("*");
                } else {
                    print!(" ");
                }
            }
        }
        println!("│");
    }
    println!("┕━━━━━┙");
}
