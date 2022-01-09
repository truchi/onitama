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

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Game {
    winner: Option<Player>,
    turn: Player,
    board: Board,
    red: Side,
    blue: Side,
    spare: Card,
    legals: Option<List<Play, PLAYS>>,
    score: Option<f32>,
}

impl Game {
    pub fn legals(&mut self) -> &[Play] {
        todo!()
    }

    pub fn play(&mut self, index: usize) {
        debug_assert!(self.winner.is_none());
        let play = *self.legals().get(index).unwrap();

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
        debug_assert!(self.legals().is_empty());
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
}
