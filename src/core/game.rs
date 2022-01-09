use super::*;
use std::ops::Deref;
use std::ops::DerefMut;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Pawns {
    pawns: [Square; PAWNS],
    len: usize,
}

impl Pawns {
    pub fn remove(&mut self, index: usize) -> Option<Square> {
        debug_assert!(index < self.len());

        self.len -= 1;

        if index == self.len {
            None
        } else {
            self.pawns.swap(index, self.len);
            Some(*self.get(index).unwrap())
        }
    }
}

impl Deref for Pawns {
    type Target = [Square];

    fn deref(&self) -> &[Square] {
        &self.pawns[..self.len]
    }
}

impl DerefMut for Pawns {
    fn deref_mut(&mut self) -> &mut [Square] {
        &mut self.pawns[..self.len]
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Side {
    pub king: Square,
    pub pawns: Pawns,
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
pub struct Legals {
    plays: [Play; PLAYS],
    len: usize,
}

impl Deref for Legals {
    type Target = [Play];

    fn deref(&self) -> &[Play] {
        &self.plays[..self.len]
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Game {
    winner: Option<Player>,
    turn: Player,
    board: Board<Option<Piece>>,
    red: Side,
    blue: Side,
    spare: Card,
    legals: Legals,
}

impl Game {
    pub fn play(&mut self, index: usize) {
        debug_assert!(self.winner.is_none());

        let play = *self.legals.get(index).unwrap();
        let side = self.side(self.turn);

        debug_assert!(self.is_play_valid(play));

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

                    if let Some(square) = pawns.remove(pawn) {
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
        debug_assert!(self.legals.is_empty());
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

    fn is_play_valid(&self, play: Play) -> bool {
        debug_assert!(play.piece.0 == self.turn);
        debug_assert!(play
            .capture
            .map(|(player, _)| player != self.turn)
            .unwrap_or(true));
        debug_assert!(self.board[play.src] == Some(play.piece));
        debug_assert!(self.board[play.dest] == play.capture);

        todo!()
    }
}
