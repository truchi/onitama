use super::*;
use std::fmt::Display;
use std::io::StdoutLock;
use x::Stylize;

const KING: &str = "♔";
const PAWN: &str = "♙";
const V: char = '│';
const H: char = '─';
const TL: char = '╭';
const TR: char = '╮';
const BL: char = '╰';
const BR: char = '╯';
const VR: char = '├';
const VL: char = '┤';
const HH: char = '━';
const HTL: char = '┍';
const HTR: char = '┑';
const HBL: char = '┕';
const HBR: char = '┙';
const RED: x::Color = x::Rgb { r: 255, g: 0, b: 0 };
const BLUE: x::Color = x::Rgb { r: 0, g: 0, b: 255 };
const BLACK: x::Color = x::Rgb { r: 0, g: 0, b: 0 };
const WHITE: x::Color = x::Rgb {
    r: 180,
    g: 180,
    b: 180,
};

pub struct GameUI {
    width:  u16,
    height: u16,
    game:   Game,
}

impl GameUI {
    const BOARD_HEIGHT: u16 = Self::BOARD_SQUARE_HEIGHT * SIZE as u16;
    const BOARD_SQUARE_HEIGHT: u16 = 3;
    const BOARD_SQUARE_WIDTH: u16 = 6;
    const BOARD_WIDTH: u16 = Self::BOARD_SQUARE_WIDTH * SIZE as u16;
    const CARD_HEIGHT: u16 = Self::CARD_SQUARE_HEIGHT * SIZE as u16 + 3;
    const CARD_SQUARE_HEIGHT: u16 = 1;
    const CARD_SQUARE_WIDTH: u16 = 3;
    const CARD_WIDTH: u16 = Self::CARD_SQUARE_WIDTH * SIZE as u16 + 2;
    const HAND_WIDTH: u16 = 2 * Self::CARD_WIDTH + Self::MARGIN;
    const MARGIN: u16 = 1;

    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width,
            height,
            game: Game::new([8, 9], [10, 11], 12),
        }
    }

    pub fn render(&self) {
        self.render_board();
        self.render_cards();
    }

    fn render_cards(&self) {
        let red = self.game[Red].cards();
        let blue = self.game[Blue].cards();

        let board_x1 = (self.width - Self::BOARD_WIDTH) / 2;
        let board_y1 = (self.height - Self::BOARD_HEIGHT) / 2;
        let board_x2 = board_x1 + Self::BOARD_WIDTH;
        let board_y2 = board_y1 + Self::BOARD_HEIGHT;

        let x1 = (self.width - Self::HAND_WIDTH) / 2;
        let x2 = x1 + Self::CARD_WIDTH + Self::MARGIN;
        let yblue = board_y1 - Self::MARGIN - Self::CARD_HEIGHT;
        let yred = board_y2 + Self::MARGIN;

        let spare = self.game.spare();
        let player = self.game.player();
        let spare_y = (self.height - Self::CARD_HEIGHT) / 2;
        let spare_x = if player == Red {
            board_x2 + Self::MARGIN
        } else {
            board_x1 - Self::MARGIN - Self::CARD_WIDTH
        };

        self.render_card(red[0], x1, yred, Red);
        self.render_card(red[1], x2, yred, Red);
        self.render_card(blue[0], x1, yblue, Blue);
        self.render_card(blue[1], x2, yblue, Blue);
        self.render_card(spare, spare_x, spare_y, player);
    }

    fn render_card(&self, card: Card, x: u16, y: u16, player: Player) {
        self.render_card_borders(x, y, player);

        let mut out = stdout();
        let lock = &mut out.lock();
        let ranks = [Five, Four, Three, Two, One];
        let files = [A, B, C, D, E];

        let x = x + 1;
        let y = y + 1;
        let board_y = if player == Red { y + 1 } else { y };

        let mut name = |lock: &mut StdoutLock| {
            if player == Red {
                to(lock, x, y);
                write!(lock, "{}", card.name);
            } else {
                let len = card.name.len();
                to(lock, x + 3 * SIZE as u16 - len as u16, y + SIZE as u16);

                for char in reverse(card.name) {
                    write!(lock, "{}", char);
                }
            }
        };

        let mut ranks = |lock: &mut StdoutLock| {
            for rank in ranks {
                let y = board_y + SIZE as u16 - rank as u16 - 1;

                to(lock, x, y);
                for file in files {
                    let center = if (file, rank) == (C, Three) { "*" } else { " " };
                    let bg = bg(file, rank);
                    let fg = match player {
                        Red => RED,
                        Blue => BLUE,
                    };

                    write!(lock, "{}", " ".on(bg));
                    write!(lock, "{}", center.with(fg).on(bg));
                    write!(lock, "{}", " ".on(bg));
                }
            }
        };

        let mut moves = |lock: &mut StdoutLock| {
            for mov in card.moves {
                let mut mov = *mov;
                if player == Blue {
                    mov.flip()
                };

                let Square(file, rank) = Square(C, Three).apply(mov).unwrap();
                let bg = bg(file, rank);
                let fg = if bg == WHITE { BLACK } else { WHITE };
                let x = x + 3 * file as u16 + 1;
                let y = board_y + SIZE as u16 - rank as u16 - 1;

                to(lock, x, y);
                write!(lock, "{}", "*".with(fg).on(bg));
            }
        };

        name(lock);
        ranks(lock);
        moves(lock);

        lock.flush().unwrap();
    }

    fn render_card_borders(&self, x: u16, mut y: u16, player: Player) {
        let mut out = stdout();
        let lock = &mut out.lock();

        let mut line = |lock: &mut StdoutLock, y, l, m, r| {
            to(lock, x, y);
            write!(lock, "{}", l);
            for _ in 0..3 * SIZE {
                write!(lock, "{}", m);
            }
            write!(lock, "{}", r);
        };

        let mut body = |lock: &mut StdoutLock, y| {
            for i in 0..=SIZE as u16 {
                let y = y + i;
                let x2 = x + 3 * SIZE as u16 + 1;
                write!(lock, "{}{}{}{}", x::MoveTo(x, y), V, x::MoveTo(x2, y), V);
            }
        };

        if player == Red {
            line(lock, y, TL, H, TR);
            body(lock, y + 1);
            line(lock, y + 2 + SIZE as u16, HBL, HH, HBR);
        } else {
            line(lock, y, HTL, HH, HTR);
            body(lock, y + 1);
            line(lock, y + 2 + SIZE as u16, BL, H, BR);
        }

        lock.flush().unwrap();
    }

    fn render_board(&self) {
        let mut out = stdout();
        let lock = &mut out.lock();
        let x = (self.width - Self::BOARD_WIDTH) / 2;
        let y = (self.height - Self::BOARD_HEIGHT) / 2;
        let size = SIZE as u16;
        let ranks = [Five, Four, Three, Two, One];
        let files = [A, B, C, D, E];

        let mut rank = |lock: &mut StdoutLock, rank: Rank| {
            let y = y + 3 * (size - 1 - rank as u16);

            to(lock, x, y);
            for file in files {
                write!(lock, "{}", "      ".on(bg(file, rank)));
            }

            to(lock, x, y + 1);
            for file in files {
                let bg = bg(file, rank);
                let center = match self.game[Square(file, rank)] {
                    None => " ".on(bg),
                    Some((Red, King)) => KING.with(RED),
                    Some((Blue, King)) => KING.with(BLUE),
                    Some((Red, _)) => PAWN.with(RED),
                    Some((Blue, _)) => PAWN.with(BLUE),
                };

                write!(lock, "{}", "  ".on(bg));
                write!(lock, "{}", center.on(bg).bold());
                write!(lock, "{}", "   ".on(bg));
            }

            to(lock, x, y + 2);
            for file in files {
                write!(lock, "{}", "      ".on(bg(file, rank)));
            }
        };

        for r in ranks {
            rank(lock, r);
        }

        lock.flush().unwrap();
    }
}

fn bg(file: File, rank: Rank) -> x::Color {
    if (file as usize + rank as usize) % 2 == 0 {
        BLACK
    } else {
        WHITE
    }
}

fn to(lock: &mut StdoutLock, x: u16, y: u16) {
    write!(lock, "{}", x::MoveTo(x, y));
}

const REVERSED_LOWER: &[char] = &[
    'ɐ', 'q', 'ɔ', 'p', 'ǝ', 'ɟ', 'ƃ', 'ɥ', 'ᴉ', 'ɾ', 'ʞ', 'l', 'ɯ', 'u', 'o', 'd', 'b', 'ɹ', 's',
    'ʇ', 'n', 'ʌ', 'ʍ', 'x', 'ʎ', 'z',
];
const REVERSED_UPPER: &[char] = &[
    'Ɐ', 'ꓭ', 'Ɔ', 'ꓷ', 'Ǝ', 'Ⅎ', '⅁', 'H', 'I', 'ꓩ', 'ꓘ', 'ꓶ', 'ꟽ', 'N', 'O', 'Ԁ', 'b', 'ꓤ', 'S',
    'ꓕ', 'Ո', 'Ʌ', 'ʍ', 'X', '⅄', 'Z',
];

pub fn reverse(str: &str) -> impl '_ + Iterator<Item = char> {
    str.bytes().rev().map(|byte| {
        if byte <= b'Z' {
            REVERSED_UPPER[(byte - b'A') as usize]
        } else {
            REVERSED_LOWER[(byte - b'a') as usize]
        }
    })
}
