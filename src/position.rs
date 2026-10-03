use std::hint::black_box;
use crate::bitboard::BitBoard;
use crate::game::{GameState, Move, MoveError};
use crate::moves::Direction::{LeftUp, RightUp};
use crate::moves::Square;
use crate::pieces::{Colour, Piece};

#[derive(Clone)]
pub struct Position {
    white_bitboards: [BitBoard; 6],
    black_bitboards: [BitBoard; 6],
    player_turn: Colour,
}

impl Position {

    pub fn new_standard_game() -> Self {
        Position {
            white_bitboards: [
                crate::game::STARTING_PAWNS_WHITE,
                crate::game::STARTING_ROOKS_WHITE,
                crate::game::STARTING_KNIGHTS_WHITE,
                crate::game::STARTING_BISHOPS_WHITE,
                crate::game::STARTING_QUEEN_WHITE,
                crate::game::STARTING_KING_WHITE,
            ],
            black_bitboards: [
                crate::game::STARTING_PAWNS_BLACK,
                crate::game::STARTING_ROOKS_BLACK,
                crate::game::STARTING_KNIGHTS_BLACK,
                crate::game::STARTING_BISHOPS_BLACK,
                crate::game::STARTING_QUEEN_BLACK,
                crate::game::STARTING_KING_BLACK,
            ],
            player_turn: Colour::White,
        }


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

    pub fn get_next_positions(&self) -> Vec<Position> {
        let pieces = self.get_pieces_and_squares(&self.player_turn);

        pieces
            .into_iter()
            .map(|p| {
                let moves = self.get_moves_single_piece(&self.player_turn, &p.0, &p.1);
                self.find_positions_from_move_Set(&p.0, &p.1, &moves)
            })
            .flatten()
        .collect()
    }

    pub fn update_bitboard(&mut self, colour: &Colour, piece: &Piece, value: BitBoard) {
        match colour {
            Colour::White => {
                self.white_bitboards[piece.to_bitboard_index()] = value;
            }
            Colour::Black => {
                self.black_bitboards[piece.to_bitboard_index()] = value;
            }
        }
    }

    fn get_piece_board(&self, colour: &Colour, piece: &Piece) -> BitBoard {
        let index: usize = piece.to_bitboard_index();
        match colour {
            Colour::White => self.white_bitboards[index].clone(),
            Colour::Black => self.black_bitboards[index].clone(),
        }
    }

    fn find_positions_from_move_Set(&self, piece: &Piece, piece_position: &Square, moves: &BitBoard) -> Vec<Position> {

        let mut remaining_moves = moves.clone().board;
        let mut next_moves = Vec::new();
        while remaining_moves != 0 {
            let zeros = remaining_moves.trailing_zeros();
            let to = BitBoard{ board: 1 << zeros };
            let piece_board = self.get_piece_board(&self.player_turn, piece);
            let piece_position = piece_position.to_bitboard();
            let result = &piece_board & &!piece_position | to.clone();

            let mut next_pos = self.clone();
            next_pos.update_bitboard(&self.player_turn, piece, result);


            match self.player_turn.other() {
                Colour::White => {
                    for i in 0..6 {
                        next_pos.white_bitboards[i] = &next_pos.white_bitboards[i] & &!&to;
                    }

                },
                Colour::Black => {

                    for i in 0..6 {
                        next_pos.black_bitboards[i] = &next_pos.black_bitboards[i] & &!&to;
                    }

                }
            }
            next_moves.push(next_pos);
            remaining_moves = remaining_moves & !&to.board;
        }
        next_moves
    }

   fn get_pieces_and_squares(&self, colour: &Colour) -> Vec<(Piece, Square)> {
       let mut board = self.get_sides_pieces(colour).board;

       let mut squares = Vec::new();
       while board != 0 {
           let sq = board.trailing_zeros();
           let sq = BitBoard {
               board: 1 << sq
           };
           let piece = match colour {
               Colour::White => &self.white_bitboards
                   .iter()
                   .enumerate()
                   .find(|b| (b.1 & &sq).board != 0)
                   .map(|b| b.0)
                   .unwrap(),
               Colour::Black => &self.black_bitboards
                   .iter()
                   .enumerate()
                   .find(|b| (b.1 & &sq).board != 0)
                   .map(|b| b.0)
                   .unwrap(),
           };
           board = board & board -1;
           let piece = match piece {
               0 => Piece::Pawn,
               1 => Piece::Rook,
               2 => Piece::Knight,
               3 => Piece::Bishop,
               4 => Piece::Queen,
               5 => Piece::King,
               _ => {
                   panic!("invalid piece nunmber");
               }
           };
           squares.push((piece, Square::try_from(sq).unwrap()));
       }
       squares
   }

    pub fn get_moves_single_piece(&self, colour: &Colour, piece: &Piece, square: &Square) -> BitBoard {
        let index: usize = piece.to_bitboard_index();
        let board = match colour {
            Colour::White => &self.white_bitboards[index],
            Colour::Black => &self.black_bitboards[index],
        }.clone();

        let pos = &square.to_bitboard() & &board;
        let pseudo_moves = piece.get_pseudo_moves(&pos.try_into().unwrap(), &self.get_all_pieces(), colour);
        return pseudo_moves & ! self.get_sides_pieces(colour);
    }

    pub fn get_piece(&self, colour: &Colour, piece: &Piece) -> BitBoard {
        let index: usize = piece.into();
        let board = match colour {
            Colour::White => &self.white_bitboards[index],
            Colour::Black => &self.black_bitboards[index],
        };
        return board.clone();
    }
}
pub fn get_pawn_attacks(position: &Position, colour: &Colour) -> BitBoard {
    let index: usize = Piece::Pawn.into();
    let board: &BitBoard = match colour {
        Colour::White => &position.white_bitboards[index],
        Colour::Black => &position.black_bitboards[index],
    };

    board.shift(LeftUp) | board.shift(RightUp)
}
