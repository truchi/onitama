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
const TINT: u8 = 75;

enum State {
    Card(usize),
    Square(usize, Square, Vec<Square>),
}

pub struct GameUI {
    width:  u16,
    height: u16,
    game:   Game,
    state:  Option<State>,
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
    const HEIGHT: u16 = Self::BOARD_HEIGHT + 2 * (Self::CARD_HEIGHT + Self::MARGIN);
    const MARGIN: u16 = 1;
    const WIDTH: u16 = Self::BOARD_WIDTH + 2 * (Self::CARD_WIDTH + Self::MARGIN);
}

impl GameUI {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width,
            height,
            game: Game::new([8, 9], [10, 11], 12),
            state: None,
        }
    }

    pub fn set_size(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
    }

    pub fn handle_click(&mut self, pos: (u16, u16), f: impl FnOnce(usize, Square, Square)) {
        fn it_contains(mut it: impl Iterator<Item = Square>, square: Square) -> bool {
            it.find(|&s| s == square).is_some()
        }

        let rect_contains = |(x1, y1, x2, y2), (x, y)| x1 <= x && x <= x2 && y1 <= y && y <= y2;
        let state_square =
            |card, src| State::Square(card, src, self.game.dests(card, src).collect::<Vec<_>>());

        let player = self.game.player();
        let board = self.board_rect();
        let [pc0, pc1] = self.cards_rect(player);
        let [oc0, oc1] = self.cards_rect(!player);
        let mut pieces = self.game[player].pieces().map(|(_, square)| square);

        enum Clicked {
            Square(Square),
            Card(Player, usize),
        };

        let clicked = if rect_contains(board, pos) {
            Some(Clicked::Square(
                Square::all()
                    .find(|&square| rect_contains(self.square_rect(square), pos))
                    .unwrap(),
            ))
        } else if rect_contains(pc0, pos) {
            Some(Clicked::Card(player, 0))
        } else if rect_contains(pc1, pos) {
            Some(Clicked::Card(player, 1))
        } else if rect_contains(oc0, pos) {
            Some(Clicked::Card(!player, 0))
        } else if rect_contains(oc1, pos) {
            Some(Clicked::Card(!player, 1))
        } else {
            None
        };

        self.state = match &self.state {
            None => match clicked {
                Some(Clicked::Card(p, card)) if p == player => Some(State::Card(card)),
                _ => None,
            },
            Some(State::Card(card)) => match clicked {
                Some(Clicked::Card(p, card)) if p == player => Some(State::Card(card)),
                Some(Clicked::Square(src)) if it_contains(pieces, src) =>
                    Some(state_square(*card, src)),
                Some(Clicked::Square(square)) => Some(State::Card(*card)),
                _ => None,
            },
            Some(State::Square(card, src, dests)) => match clicked {
                Some(Clicked::Card(p, card)) if p == player => Some(State::Card(card)),
                Some(Clicked::Square(src)) if it_contains(pieces, src) =>
                    Some(state_square(*card, src)),
                Some(Clicked::Square(dest)) if it_contains(dests.iter().copied(), dest) =>
                    return f(*card, *src, dest),
                Some(Clicked::Square(square)) => Some(State::Card(*card)),
                _ => None,
            },
        };

        self.render();
    }

    pub fn render(&self) {
        let mut out = stdout();
        let lock = &mut out.lock();

        self.clear(lock);
        self.render_board(lock);
        self.render_cards(lock);

        lock.flush().unwrap();
    }
}

impl GameUI {
    fn clear(&self, lock: &mut StdoutLock) {
        write!(lock, "{}", x::Clear(x::ClearType::All));
    }

    fn render_cards(&self, lock: &mut StdoutLock) {
        let red = self.game[Red].cards();
        let blue = self.game[Blue].cards();

        let [(rx1, ry1, ..), (rx2, ry2, ..)] = self.cards_rect(Red);
        let [(bx1, by1, ..), (bx2, by2, ..)] = self.cards_rect(Blue);
        let (sx, sy, ..) = self.spare_rect();
        let player = self.game.player();
        let spare = self.game.spare();

        let selected = match self.state {
            None => None,
            Some(State::Card(card)) => Some(card),
            Some(State::Square(card, ..)) => Some(card),
        };

        let is_selected = |p, card| selected == Some(card) && player == p;

        self.render_card(lock, red[0], rx1, ry1, Red, is_selected(Red, 0));
        self.render_card(lock, red[1], rx2, ry2, Red, is_selected(Red, 1));
        self.render_card(lock, blue[0], bx1, by1, Blue, is_selected(Blue, 0));
        self.render_card(lock, blue[1], bx2, by2, Blue, is_selected(Blue, 1));
        self.render_card(lock, spare, sx, sy, player, false);
    }

    fn render_card(
        &self,
        lock: &mut StdoutLock,
        card: Card,
        x: u16,
        y: u16,
        player: Player,
        is_selected: bool,
    ) {
        self.render_card_borders(lock, x, y, player, is_selected);

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

        let mut board = |lock: &mut StdoutLock| {
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
        board(lock);
        moves(lock);
    }

    fn render_card_borders(
        &self,
        lock: &mut StdoutLock,
        x: u16,
        mut y: u16,
        player: Player,
        is_selected: bool,
    ) {
        let color = if is_selected {
            match player {
                Red => RED,
                Blue => BLUE,
            }
        } else {
            x::Color::Reset
        };

        let mut line = |lock: &mut StdoutLock, y, l: char, m: char, r: char| {
            to(lock, x, y);
            write!(lock, "{}", l.with(color));
            for _ in 0..3 * SIZE {
                write!(lock, "{}", m.with(color));
            }
            write!(lock, "{}", r.with(color));
        };

        let mut body = |lock: &mut StdoutLock, y| {
            for i in 0..=SIZE as u16 {
                let y = y + i;
                let x2 = x + 3 * SIZE as u16 + 1;
                let v = V.with(color);
                write!(lock, "{}{}{}{}", x::MoveTo(x, y), v, x::MoveTo(x2, y), v);
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
    }

    fn render_board(&self, lock: &mut StdoutLock) {
        let x = (self.width - Self::BOARD_WIDTH) / 2;
        let y = (self.height - Self::BOARD_HEIGHT) / 2;
        let size = SIZE as u16;
        let ranks = [Five, Four, Three, Two, One];
        let files = [A, B, C, D, E];

        let tinted_bg = |file, rank| {
            let bg = bg(file, rank);

            if self.is_active(Square(file, rank)) {
                tint(bg, self.game.player())
            } else {
                bg
            }
        };

        for rank in ranks {
            let y = y + 3 * (size - 1 - rank as u16);

            to(lock, x, y);
            for file in files {
                write!(lock, "{}", "      ".on(tinted_bg(file, rank)));
            }

            to(lock, x, y + 1);
            for file in files {
                let bg = tinted_bg(file, rank);
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
                write!(lock, "{}", "      ".on(tinted_bg(file, rank)));
            }
        }
    }
}

impl GameUI {
    fn is_active(&self, square: Square) -> bool {
        if let Some(State::Square(_, src, dests)) = &self.state {
            square == *src || dests.contains(&square)
        } else {
            false
        }
    }

    fn board_rect(&self) -> (u16, u16, u16, u16) {
        let x = (self.width - Self::BOARD_WIDTH) / 2;
        let y = (self.height - Self::BOARD_HEIGHT) / 2;

        (x, y, x + Self::BOARD_WIDTH, y + Self::BOARD_HEIGHT)
    }

    fn cards_rect(&self, player: Player) -> [(u16, u16, u16, u16); HAND] {
        let (_, board_y1, _, board_y2) = self.board_rect();
        let x1 = (self.width - Self::HAND_WIDTH) / 2;
        let x2 = x1 + Self::MARGIN + Self::CARD_WIDTH;
        let y = if player == Red {
            board_y2 + Self::MARGIN
        } else {
            board_y1 - Self::MARGIN - Self::CARD_HEIGHT
        };

        [
            (x1, y, x1 + Self::CARD_WIDTH, y + Self::CARD_HEIGHT),
            (x2, y, x2 + Self::CARD_WIDTH, y + Self::CARD_HEIGHT),
        ]
    }

    fn spare_rect(&self) -> (u16, u16, u16, u16) {
        let (board_x1, _, board_x2, _) = self.board_rect();
        let y = (self.height - Self::CARD_HEIGHT) / 2;
        let x = if self.game.player() == Red {
            board_x2 + Self::MARGIN
        } else {
            board_x1 - Self::MARGIN - Self::CARD_WIDTH
        };

        (x, y, x + Self::CARD_WIDTH, y + Self::CARD_HEIGHT)
    }

    fn square_rect(&self, square: Square) -> (u16, u16, u16, u16) {
        let (board_x, board_y, ..) = self.board_rect();

        let file = square.file() as u16;
        let rank = SIZE as u16 - square.rank() as u16 - 1;

        let x = board_x + file * Self::BOARD_SQUARE_WIDTH;
        let y = board_y + rank * Self::BOARD_SQUARE_HEIGHT;

        (
            x,
            y,
            x + Self::BOARD_SQUARE_WIDTH,
            y + Self::BOARD_SQUARE_HEIGHT,
        )
    }
}

fn bg(file: File, rank: Rank) -> x::Color {
    if (file as usize + rank as usize) % 2 == 0 {
        BLACK
    } else {
        WHITE
    }
}

fn tint(mut color: x::Color, player: Player) -> x::Color {
    if let x::Color::Rgb { r, g, b } = &mut color {
        let channel = match player {
            Red => r,
            Blue => b,
        };

        *channel = channel.saturating_add(TINT);
    } else {
        unreachable!();
    }

    color
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

fn reverse(str: &str) -> impl '_ + Iterator<Item = char> {
    str.bytes().rev().map(|byte| {
        if byte <= b'Z' {
            REVERSED_UPPER[(byte - b'A') as usize]
        } else {
            REVERSED_LOWER[(byte - b'a') as usize]
        }
    })
}
