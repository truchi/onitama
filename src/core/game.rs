use super::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Side {
    pub pieces: [Option<Square>; SIZE],
    pub cards: [Card; HAND],
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
            .enumerate()
            .map(|(c, card)| {
                card.moves
                    .iter()
                    .enumerate()
                    .map(move |(m, mov)| ((c, *card), (m, *mov)))
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
    pub piece: Piece,
    pub card: (usize, Card),
    pub r#move: (usize, Move),
    pub src: Square,
    pub dest: Square,
    pub capture: Option<Piece>,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Game {
    winner: Option<Player>,
    player: Player,
    board: Board,
    red: Side,
    blue: Side,
    spare: Card,
}

impl Game {
    pub fn plays(&self) -> Option<Vec<Play>> {
        let mut plays = vec![];
        let player = self.player;
        let side = self.side(player);

        for (piece, src) in side.pieces(player) {
            for (card, r#move) in side.moves() {
                if let Some(dest) = src.apply(r#move.1) {
                    let capture = self.board[dest];

                    if matches!(capture, Some(piece) if piece.player() == player) {
                        continue;
                    }

                    plays.push(Play {
                        piece,
                        card,
                        r#move,
                        src,
                        dest,
                        capture,
                    });
                }
            }
        }

        if plays.is_empty() {
            None
        } else {
            Some(plays)
        }
    }

    pub fn play(&mut self, play: Play) {
        debug_assert!(self.winner.is_none());
        debug_assert!(self.is_play_legal(play));

        // Update board
        self.board[play.src] = None;
        self.board[play.dest] = Some(play.piece);

        // Update pieces
        *self.side_mut(self.player).square_mut(play.piece) = Some(play.dest);

        // Update hand
        self.discard_unchecked(play.card.0);

        // Update winner
        let stone = play.capture == Some(Piece::king(!self.player));
        let stream = self.board[Square::king(!self.player)] == Some(Piece::king(self.player));

        if stone || stream {
            self.winner = Some(self.player);
        }
    }

    pub fn discard(&mut self, card: usize) {
        debug_assert!(self.winner.is_none());
        debug_assert!(self.plays().is_none());

        self.discard_unchecked(card);
    }

    fn discard_unchecked(&mut self, card: usize) {
        debug_assert!(card < HAND);

        std::mem::swap(&mut self.spare, {
            &mut match self.turn {
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

    fn is_play_legal(&self, play: Play) -> bool {
        let side = self.side(self.turn);

        // Is it player's turn?
        if play.piece.player() != self.turn {
            return false;
        }

        // Is card OK?
        if side.cards[play.card.0] != play.card.1 {
            return false;
        }

        // Is move OK?
        if play.card.1.r#moves[play.r#move.0] != play.r#move.1 {
            return false;
        }

        // Is src correct?
        if Some(play.src) != *side.square(play.piece) {
            return false;
        }

        // Is move in board?
        if let Some(dest) = play.src.apply(play.r#move.1) {
            // Is dest correct?
            if play.dest != dest {
                return false;
            }

            // Is capture correct?
            if self.board[dest] != play.capture {
                return false;
            }

            // Is capture other player's piece?
            if let Some(piece) = play.capture {
                if piece.player() != !self.turn {
                    return false;
                }
            }
        } else {
            return false;
        }

        true
    }
}
