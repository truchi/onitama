use super::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Side {
    pub king: Square,
    pub pawns: List<Square, PAWNS>,
    pub hand: [Card; HAND],
}

impl Side {
    pub fn square(&self, piece: PieceType) -> &Square {
        match piece {
            King => &self.king,
            Pawn(pawn) => self.pawns.get(pawn).unwrap(),
        }
    }

    pub fn square_mut(&mut self, piece: PieceType) -> &mut Square {
        match piece {
            King => &mut self.king,
            Pawn(pawn) => self.pawns.get_mut(pawn).unwrap(),
        }
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
    turn: Player,
    board: Board,
    red: Side,
    blue: Side,
    spare: Card,
}

impl Game {
    pub fn plays(&mut self) -> Vec<Play> {
        todo!()
    }

    pub fn play(&mut self, play: Play) {
        debug_assert!(self.winner.is_none());
        debug_assert!(self.is_play_legal(play));

        // Update board
        self.board[play.src] = None;
        self.board[play.dest] = Some(play.piece);

        if self.board[Square::king(!self.turn)] == Some((self.turn, King)) {
            self.winner = Some(self.turn);
        }

        // Update pieces
        *self.side_mut(self.turn).square_mut(play.piece.1) = play.dest;

        if let Some((_, piece)) = play.capture {
            match piece {
                King => {
                    self.winner = Some(self.turn);
                }
                Pawn(pawn) => {
                    let mut pawns = self.side_mut(!self.turn).pawns;

                    if let Some(&square) = pawns.swap_remove(pawn) {
                        debug_assert!(self.board[square] == Some((!self.turn, Pawn(pawns.len()))));
                        self.board[square] = Some((!self.turn, Pawn(pawn)));
                    }
                }
            }
        }

        // Update hand
        self.discard_unchecked(play.card.0);

        ()
    }

    pub fn discard(&mut self, card: usize) {
        debug_assert!(self.winner.is_none());
        debug_assert!(self.plays().is_empty());
        self.discard_unchecked(card);
    }

    fn discard_unchecked(&mut self, card: usize) {
        std::mem::swap(&mut self.spare, {
            &mut match self.turn {
                Red => &mut self.red,
                Blue => &mut self.blue,
            }
            .hand[card]
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
        if play.piece.0 != self.turn {
            return false;
        }

        // Is pawn alive?
        if let Pawn(pawn) = play.piece.1 {
            if pawn >= side.pawns.len() {
                return false;
            }
        }

        // Is card OK?
        if side.hand[play.card.0] != play.card.1 {
            return false;
        }

        // Is move OK?
        if play.card.1.r#moves[play.r#move.0] != play.r#move.1 {
            return false;
        }

        // Is src correct?
        if play.src != *side.square(play.piece.1) {
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
            if let Some((player, _)) = play.capture {
                if player != !self.turn {
                    return false;
                }
            }
        } else {
            return false;
        }

        true
    }
}
