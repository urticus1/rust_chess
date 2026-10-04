use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use crate::bitboard::{BitBoard};
use crate::{ ROW_2, ROW_6, ROW_7};
use crate::moves::Direction::{LeftDown, LeftUp, RightDown, RightUp};
use crate::moves::Square;
use crate::pieces::{Colour, Piece};
use crate::position::Position;

pub(crate) const STARTING_PAWNS_BLACK: BitBoard = ROW_7;
pub(crate) const STARTING_PAWNS_WHITE: BitBoard = ROW_2;

pub(crate) const STARTING_ROOKS_WHITE: BitBoard = BitBoard(0b_10000001 );
pub(crate) const STARTING_KNIGHTS_WHITE: BitBoard = BitBoard(0b_01000010 );

pub(crate) const STARTING_BISHOPS_WHITE: BitBoard = BitBoard(0b_00100100 );
pub(crate) const STARTING_QUEEN_WHITE: BitBoard = BitBoard(0b_00001000 );
pub(crate) const STARTING_KING_WHITE: BitBoard = BitBoard(0b_00010000 );

pub(crate) const STARTING_ROOKS_BLACK: BitBoard = BitBoard(STARTING_ROOKS_WHITE.0 << 8 * 7 );
pub(crate) const STARTING_KNIGHTS_BLACK: BitBoard = BitBoard(STARTING_KNIGHTS_WHITE.0 << 8 * 7 );
pub(crate) const STARTING_BISHOPS_BLACK: BitBoard = BitBoard(STARTING_BISHOPS_WHITE.0 << 8 * 7 );
pub(crate) const STARTING_QUEEN_BLACK: BitBoard = BitBoard(STARTING_QUEEN_WHITE.0 << 8 * 7 );
pub(crate) const STARTING_KING_BLACK: BitBoard = BitBoard(STARTING_KING_WHITE.0 << 8 * 7 );


pub struct GameState {
    pos: Position,
}

impl GameState {

    pub fn new_standard_game() -> Self {
        GameState {
            pos: Position::new_standard_game()
        }
    }

    pub fn make_move(&mut self, m: Move) {
        let current = &self.pos;
        let res = current.make_move(m);
        match res {
            Ok(position) => self.pos = position,
            Err(e) => println!("{}", e)
        }
    }

    pub fn print(&self) {
        self.pos.get_all_pieces().print();
    }
}

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

pub enum Move {
    Normal(NormalMove),
    Castle(CastleSide),
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