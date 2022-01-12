use super::*;
use std::ops::Index;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Side {
    pub pieces: [Option<Square>; SIZE],
    pub cards: [usize; HAND],
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
        let side = self.side(player);

        for (_, src) in side.pieces() {
            for ((card, _), (_, r#move)) in side.moves() {
                if let Some(dest) = src.apply(r#move) {
                    if matches!(self[dest], Some((p, _)) if p == player) {
                        continue;
                    }

                    plays.push(Play { card, src, dest });
                }
            }
        }

        plays
    }

    pub fn play(&mut self, play: Play) {
        debug_assert!(self.winner.is_none());

        let (player, piece) = self[play.src].unwrap();
        let capture = self[play.dest];

        debug_assert!(self.player == player);

        // Update board
        self.board[play.src] = None;
        self.board[play.dest] = Some((player, piece));

        // Update pieces
        *self.side_mut(player).square_mut(piece) = Some(play.dest);

        // Update hand
        self.discard_unchecked(play.card);

        // Update winner
        let stone = capture == Some((!player, King));
        let stream = self[Square::king(!player)] == Some((player, King));

        if stone || stream {
            self.winner = Some(player);
        }
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
