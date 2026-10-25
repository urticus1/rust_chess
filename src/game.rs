use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use crate::squares::Square;
use crate::pieces::{Colour, Piece};
use crate::position::Position;

pub struct GameState {
    pub pos: Position,
}

impl GameState {

    pub fn new_standard_game() -> Self {
        GameState {
            pos: Position::new_standard_game()
        }
    }

    pub fn make_move(&mut self, m: Move) -> Result<(), MoveError> {
        let current = &self.pos;

        match m {
            Move::Normal(ref mv) => {
                if let Err(err) = self.validate_normal_move(mv) {
                    return Err(err);
                }
                if let Err(err) = self.pos.make_normal_move(mv) {
                    return Err(err);
                }
            },
            Move::Castle(side, colour) => {
                if let Err(err) = self.validate_castle_move(colour, side) {
                    return Err(err);
                }
                self.pos.make_castle_move(side);
            }
        }
        let res = current.make_move(m);
        return match res {
            Ok(position) => {
                self.pos = position;
                Ok(())
            },
            Err(e) => Err(e)
        };
    }

    fn validate_castle_move(&self, colour: Colour, side: CastleSide) -> Result<(), MoveError> {
        if !self.pos.can_castle(colour, side) {
            return Err(MoveError::new("Cannot castle"))
        };
        Ok(())
    }

    fn validate_normal_move(&self, mv: &NormalMove) -> Result<(), MoveError> {
        let from = mv.from.to_bitboard();
        let to = mv.to.to_bitboard();

        if (from & self.pos.get_piece(self.pos.player_turn, mv.piece)).is_empty() {
            return Err(MoveError::new("No correct colour piece on square"));
        }

        let pseudo_moves = self.pos.get_pseudo_moves_single_piece_validated(self.pos.player_turn, mv.piece, mv.from);
        if pseudo_moves.is_none() {
            return Err(MoveError::new("No correct position piece on square"));
        }
        let pseudo_moves = pseudo_moves.unwrap() & !self.pos.get_sides_pieces(self.pos.player_turn);


        if (to & pseudo_moves).is_empty() {
            return Err(MoveError::new("Not a valid destination psuedo move"));
        }
        Ok(())
    }

    pub fn print(&self) {
        print!("{}", self.pos.get_all_pieces());
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NormalMove {
    pub from: Square,
    pub to: Square,
    pub piece: Piece,
}

#[derive(Copy, Clone, PartialEq, Debug, Eq, Hash)]
pub enum CastleSide {
    King = 0,
    Queen = 1,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Move {
    Normal(NormalMove),
    Castle(CastleSide, Colour),
}

#[derive(Debug)]
pub struct MoveError {
    message: String,
}

impl MoveError {
    pub fn new(message: &str) -> Self {
        MoveError {
            message: message.to_string(),
        }
    }
}

impl Display for MoveError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl Error for MoveError {
    
}