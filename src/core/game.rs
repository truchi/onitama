use super::*;
use std::ops::Index;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Side {
    pub pieces: [Option<Square>; SIZE],
    pub cards:  [usize; HAND],
}

impl Side {
    pub fn new(player: Player, cards: [usize; HAND]) -> Self {
        macro_rules! pieces {
            ($rank:ident $($file:ident)*) => { [$(Some(Square($file, $rank))),*] };
        }

        let rank = player.rank();
        Self {
            pieces: pieces!(rank A B C D E),
            cards,
        }
    }

    pub fn pieces(&self) -> impl '_ + Iterator<Item = (Piece, Square)> {
        self.pieces
            .into_iter()
            .enumerate()
            .filter_map(|(i, square)| {
                if let Some(square) = square {
                    Some((i, square))
                } else {
                    None
                }
            })
            .map(move |(i, square)| (Piece::from(i), square))
    }

    pub fn squares(&self) -> impl '_ + Iterator<Item = Square> {
        self.pieces.into_iter().filter_map(|square| square)
    }

    pub fn cards(&self) -> [Card; HAND] {
        [CARDS[self.cards[0]], CARDS[self.cards[1]]]
    }

    pub fn square(&self, piece: Piece) -> &Option<Square> {
        &self.pieces[piece.index()]
    }

    pub fn square_mut(&mut self, piece: Piece) -> &mut Option<Square> {
        &mut self.pieces[piece.index()]
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Play {
    Card {
        card: usize,
        src:  Square,
        dest: Square,
    },
    Discard(usize),
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Game {
    winner: Option<Player>,
    player: Player,
    board:  Board,
    red:    Side,
    blue:   Side,
    spare:  usize,
}

impl Game {
    pub fn new(red: [usize; HAND], blue: [usize; HAND], spare: usize) -> Self {
        Self {
            winner: None,
            player: CARDS[spare].stamp,
            board: Board::default(),
            red: Side::new(Red, red),
            blue: Side::new(Blue, blue),
            spare,
        }
    }

    pub fn player(&self) -> Player {
        self.player
    }

    pub fn side(&self, player: Player) -> &Side {
        match player {
            Red => &self.red,
            Blue => &self.blue,
        }
    }

    pub fn spare(&self) -> Card {
        CARDS[self.spare]
    }

    pub fn pieces(&self, player: Player) -> impl '_ + Iterator<Item = (Piece, Square)> {
        self[player].pieces()
    }

    pub fn distance(&self, player: Player) -> u8 {
        let king = self[(player, King)].unwrap();
        let square = Square::king(!player);

        let king = (king.file() as i8, king.rank() as i8);
        let square = (square.file() as i8, square.rank() as i8);

        let distance = ((king.0 - square.0).abs(), (king.1 - square.1).abs());
        distance.0.max(distance.1) as u8
    }

    pub fn plays(&self) -> Vec<Play> {
        let mut plays = vec![];
        let player = self.player;
        let flipper = player.flipper();
        let side = self.side(player);
        let cards = side.cards();
        let squares = side.squares();
        let is_valid = move |square| !matches!(self[square], Some((p, _)) if p == player);

        for src in squares {
            for (c, card) in cards.iter().enumerate() {
                for mov in card.moves.iter().map(flipper) {
                    if let Some(dest) = src.apply(mov) {
                        if is_valid(dest) {
                            plays.push(Play::Card { card: c, src, dest });
                        }
                    }
                }
            }
        }

        if plays.is_empty() {
            for card in 0..HAND {
                plays.push(Play::Discard(card));
            }
        }

        plays
    }

    pub fn dests(&self, card: usize, src: Square) -> impl '_ + Iterator<Item = Square> {
        let player = self.player;
        let side = self.side(player);
        let card = side.cards()[card];
        let mut moves = card.moves.iter().map(player.flipper());

        let has_piece = move |square| matches!(self[square], Some((p, _)) if p == player);
        debug_assert!(has_piece(src));

        std::iter::from_fn(move || loop {
            if let Some(dest) = src.apply(moves.next()?).filter(|&dest| !has_piece(dest)) {
                return Some(dest);
            }
        })
    }

    pub fn play(&mut self, play: Play) -> Option<Player> {
        debug_assert!(self.winner.is_none());

        let discard = match play {
            Play::Card { card, src, dest } => {
                let (player, piece) = self[src].unwrap();
                let capture = self[dest];

                debug_assert!(self.player == player);

                // Update board
                self.board[src] = None;
                self.board[dest] = Some((player, piece));

                // Update pieces
                *self.side_mut(player).square_mut(piece) = Some(dest);

                // Update hand
                self.discard_unchecked(card);

                // Update winner
                let stone = capture == Some((!player, King));
                let stream = self[Square::king(!player)] == Some((player, King));

                if stone || stream {
                    self.winner = Some(player);
                }

                card
            }
            Play::Discard(card) => card,
        };

        // Update hand
        self.discard_unchecked(discard);

        // Update player
        self.player = !self.player;

        self.winner
    }

    pub fn discard(&mut self, card: usize) {
        debug_assert!(self.winner.is_none());
        debug_assert!(self.plays().is_empty());

        self.discard_unchecked(card);
    }
}

impl Game {
    fn discard_unchecked(&mut self, card: usize) {
        debug_assert!(card < HAND);

        std::mem::swap(&mut self.spare, {
            &mut match self.player {
                Red => &mut self.red,
                Blue => &mut self.blue,
            }
            .cards[card]
        });
    }

    fn side_mut(&mut self, player: Player) -> &mut Side {
        match player {
            Red => &mut self.red,
            Blue => &mut self.blue,
        }
    }
}

impl Index<Square> for Game {
    type Output = Option<(Player, Piece)>;

    fn index(&self, square: Square) -> &Option<(Player, Piece)> {
        &self.board[square]
    }
}

impl Index<Player> for Game {
    type Output = Side;

    fn index(&self, player: Player) -> &Side {
        self.side(player)
    }
}

impl Index<(Player, Piece)> for Game {
    type Output = Option<Square>;

    fn index(&self, (player, piece): (Player, Piece)) -> &Option<Square> {
        self.side(player).square(piece)
    }
}
