use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use crate::bitboard::{BitBoard};
use crate::{ ROW_2, ROW_6, ROW_7};
use crate::moves::Direction::{LeftUp, RightUp};
use crate::moves::Square;
use crate::pieces::{Colour, Piece};

pub(crate) const STARTING_PAWNS_BLACK: BitBoard = ROW_7;
pub(crate) const STARTING_PAWNS_WHITE: BitBoard = ROW_2;

pub(crate) const STARTING_ROOKS_WHITE: BitBoard = BitBoard{ board: 0b_10000001 };
pub(crate) const STARTING_KNIGHTS_WHITE: BitBoard = BitBoard{ board: 0b_01000010 };

pub(crate) const STARTING_BISHOPS_WHITE: BitBoard = BitBoard{ board: 0b_00100100 };
pub(crate) const STARTING_QUEEN_WHITE: BitBoard = BitBoard{ board: 0b_00001000 };
pub(crate) const STARTING_KING_WHITE: BitBoard = BitBoard{ board: 0b_00010000 };

pub(crate) const STARTING_ROOKS_BLACK: BitBoard = BitBoard{ board: STARTING_ROOKS_WHITE.board << 8 * 7 };
pub(crate) const STARTING_KNIGHTS_BLACK: BitBoard = BitBoard { board: STARTING_KNIGHTS_WHITE.board << 8 * 7 };
pub(crate) const STARTING_BISHOPS_BLACK: BitBoard = BitBoard { board: STARTING_BISHOPS_WHITE.board << 8 * 7 };
pub(crate) const STARTING_QUEEN_BLACK: BitBoard = BitBoard { board: STARTING_QUEEN_WHITE.board << 8 * 7 };
pub(crate) const STARTING_KING_BLACK: BitBoard = BitBoard { board: STARTING_KING_WHITE.board << 8 * 7 };


pub struct GameState {
    white_bitboards: [BitBoard; 6],
    black_bitboards: [BitBoard; 6],
    player_turn: Colour,
}

impl GameState {

    pub fn find_piece_squares(&self, piece: &Piece, colour: &Colour) -> Vec<Square> {
        let mut board: u64 = self.get_piece(colour, piece).board;

        let mut result = vec![];
        while board != 0 {
            let lowest_occupied = board.trailing_zeros();
            let square = 1 << lowest_occupied;
            let square = BitBoard {
                board: square
            };

            result.push(Square::try_from(square).unwrap());
            board &= board - 1;
        }

        result
    }
    pub fn get_sides_pieces(&self, colour: &Colour) -> BitBoard {
        let boards = match colour {
            Colour::White => &self.white_bitboards,
            Colour::Black => &self.black_bitboards,
        };
        boards.iter().cloned().reduce(|a,b| a | b).unwrap()
    }

    pub fn get_all_pieces(&self) -> BitBoard {
       self.white_bitboards.iter().cloned().reduce(|a,b| a | b).unwrap()
       | self.black_bitboards.iter().cloned().reduce(|a,b| a | b).unwrap()
    }

    pub fn get_piece(&self, colour: &Colour, piece: &Piece) -> BitBoard {
        let index: usize = piece.into();
        let board = match colour {
            Colour::White => &self.white_bitboards[index],
            Colour::Black => &self.black_bitboards[index],
        };
        return board.clone();
    }

    pub fn get_attacks(&self, colour: &Colour) -> BitBoard {
        let attacks = self.get_moves(colour, &Piece::Knight)
            | self.get_moves(colour, &Piece::Bishop)
            | self.get_moves(colour, &Piece::Rook)
            | self.get_moves(colour, &Piece::Queen)
            | self.get_moves(colour, &Piece::King);

        attacks | self.get_pawn_attacks(colour)
    }

    pub fn get_pawn_attacks(&self, colour: &Colour) -> BitBoard {
        let index: usize = Piece::Pawn.into();
        let board: &BitBoard = match colour {
            Colour::White => &self.white_bitboards[index],
            Colour::Black => &self.black_bitboards[index],
        };

        board.shift(LeftUp) | board.shift(RightUp)
    }

    pub fn make_move(&mut self, mv: &Move) -> Result<(), MoveError> {
        let from = mv.from.to_bitboard();
        let to = mv.to.to_bitboard();

        if (&from & &self.get_piece(&self.player_turn, &mv.piece)).board == 0 {
            return Err(MoveError{
                message: "No correct colour piece on square".to_string(),
            });
        }

        let pseudo_moves = self.get_moves_single_piece(&self.player_turn, &mv.piece, &mv.from);
        let pseudo_moves = pseudo_moves & !self.get_sides_pieces(&self.player_turn);


        if (&to & &pseudo_moves).board == 0 {
            return Err(MoveError {
                message: "Not a valid destination psuedo move".to_string(),
            });
        }
        let index: usize = mv.piece.to_bitboard_index();



        match &self.player_turn {
            Colour::White => {
                let mut board = self.white_bitboards[index].clone();
                board = &(&board ^ &from) | &to;
                self.white_bitboards[index] = board.clone();
                let opposition_attacks = self.get_attacks(&self.player_turn.other());
                if (opposition_attacks & self.get_piece(&self.player_turn, &Piece::King)).board != 0 {
                   self.white_bitboards[index] = board | from & !to;
                    return Err(MoveError {
                        message: "King in check".to_string()
                    })
                }
            },
            Colour::Black => {
                let mut board = self.black_bitboards[index].clone();
                board = &(&board ^ &from) | &to;
                self.black_bitboards[index] = board.clone();
                let opposition_attacks = self.get_attacks(&self.player_turn.other());
                if (opposition_attacks & self.get_piece(&self.player_turn, &Piece::King)).board != 0 {
                    self.black_bitboards[index] = board | from & !to;
                    return Err(MoveError {
                        message: "King in check".to_string()
                    })
                }

            }
        }

        self.player_turn = self.player_turn.other();

        Ok(())
    }

    pub fn get_moves_single_piece(&self, colour: &Colour, piece: &Piece, square: &Square) -> BitBoard {
        let index: usize = piece.into();
        let board = match colour {
            Colour::White => &self.white_bitboards[index],
            Colour::Black => &self.black_bitboards[index],
        }.clone();

        let pos = &square.to_bitboard() & &board;
        piece.get_pseudo_moves(&pos.try_into().unwrap(), &self.get_all_pieces(), colour)
    }

    pub fn get_moves(&self, colour: &Colour, piece: &Piece) -> BitBoard {
        let squares = self.find_piece_squares(piece, colour);
        squares.into_iter().map(|sq| self.get_moves_single_piece(colour, piece, &sq)).reduce(|a,b| a | b).unwrap()
    }

    pub fn new_standard_game() -> Self {
        GameState {
            white_bitboards: [
                STARTING_PAWNS_WHITE,
                STARTING_ROOKS_WHITE,
                STARTING_KNIGHTS_WHITE,
                STARTING_BISHOPS_WHITE,
                STARTING_QUEEN_WHITE,
                STARTING_KING_WHITE,
            ],
            black_bitboards: [
                STARTING_PAWNS_BLACK,
                STARTING_ROOKS_BLACK,
                STARTING_KNIGHTS_BLACK,
                STARTING_BISHOPS_BLACK,
                STARTING_QUEEN_BLACK,
                STARTING_KING_BLACK,
            ],
            player_turn: Colour::White,
        }
    }
}

pub struct Move {
    pub from: Square,
    pub to: Square,
    pub piece: Piece,
}

pub struct MoveError {
    message: String,
}

impl Debug for MoveError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}


impl Display for MoveError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl Error for MoveError {
    
}