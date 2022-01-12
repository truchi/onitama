use super::*;
use std::ops::Index;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Side {
    pub pieces: [Option<Square>; SIZE],
    pub cards: [usize; HAND],
}

impl Side {
    pub fn pieces(&self, player: Player) -> impl '_ + Iterator<Item = (Piece, Square)> {
        self.pieces
            .into_iter()
            .filter_map(|s| s)
            .enumerate()
            .map(move |(i, square)| (Piece::from((i, player)), square))
    }

    pub fn moves(&self) -> impl '_ + Iterator<Item = ((usize, Card), (usize, Move))> {
        self.cards
            .iter()
            .map(|&card| CARDS[card])
            .enumerate()
            .map(|(c, card)| {
                card.moves
                    .iter()
                    .enumerate()
                    .map(move |(m, &mov)| ((c, card), (m, mov)))
            })
            .flatten()
    }

    pub fn square(&self, piece: Piece) -> &Option<Square> {
        &self.pieces[piece.index()]
    }

    pub fn square_mut(&mut self, piece: Piece) -> &mut Option<Square> {
        &mut self.pieces[piece.index()]
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Play {
    pub card: usize,
    pub src: Square,
    pub dest: Square,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Game {
    winner: Option<Player>,
    player: Player,
    board: Board,
    red: Side,
    blue: Side,
    spare: usize,
    plays: Option<Box<[Play]>>,
}

impl Game {
    pub fn pieces(&self, player: Player) -> impl '_ + Iterator<Item = (Piece, Square)> {
        self[player].pieces(player)
    }

    pub fn distance(&self, player: Player) -> u8 {
        let king = self[Piece::king(player)].unwrap();
        let square = Square::king(!player);

        let king = (king.file() as i8, king.rank() as i8);
        let square = (square.file() as i8, square.rank() as i8);

        let distance = ((king.0 - square.0).abs(), (king.1 - square.1).abs());
        distance.0.max(distance.1) as u8
    }

    pub fn plays(&mut self) -> &[Play] {
        if self.plays.is_none() {
            self.plays = Some(self.compute_plays().into());
        }

        &(&self.plays.as_ref()).unwrap()[..]
    }

    pub fn play(&mut self, play: Play) {
        debug_assert!(self.winner.is_none());

        let piece = self[play.src].unwrap();
        let capture = self[play.dest];

        // Update board
        self.board[play.src] = None;
        self.board[play.dest] = Some(piece);

        // Update pieces
        *self.side_mut(self.player).square_mut(piece) = Some(play.dest);

        // Update hand
        self.discard_unchecked(play.card);

        // Update winner
        let stone = capture == Some(Piece::king(!self.player));
        let stream = self[Square::king(!self.player)] == Some(Piece::king(self.player));

        if stone || stream {
            self.winner = Some(self.player);
        }
    }

    pub fn discard(&mut self, card: usize) {
        debug_assert!(self.winner.is_none());
        debug_assert!(self.plays().is_empty());

        self.discard_unchecked(card);
    }
}

impl Game {
    fn compute_plays(&self) -> Vec<Play> {
        let mut plays = vec![];
        let player = self.player;
        let side = self.side(player);

        for (piece, src) in side.pieces(player) {
            for ((card, _), (_, r#move)) in side.moves() {
                if let Some(dest) = src.apply(r#move) {
                    if matches!(self[dest], Some(piece) if piece.player() == player) {
                        continue;
                    }

                    plays.push(Play { card, src, dest });
                }
            }
        }

        plays
    }

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

    pub fn side(&self, player: Player) -> &Side {
        match player {
            Red => &self.red,
            Blue => &self.blue,
        }
    }

    pub fn side_mut(&mut self, player: Player) -> &mut Side {
        match player {
            Red => &mut self.red,
            Blue => &mut self.blue,
        }
    }
}

impl Index<Square> for Game {
    type Output = Option<Piece>;

    fn index(&self, square: Square) -> &Option<Piece> {
        &self.board[square]
    }
}

impl Index<Player> for Game {
    type Output = Side;

    fn index(&self, player: Player) -> &Side {
        self.side(player)
    }
}

impl Index<Piece> for Game {
    type Output = Option<Square>;

    fn index(&self, piece: Piece) -> &Option<Square> {
        self.side(piece.player()).square(piece)
    }
}
