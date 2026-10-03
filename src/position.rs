use std::hint::black_box;
use crate::bitboard::BitBoard;
use crate::game::{GameState, Move, MoveError};
use crate::moves::Direction::{LeftDown, LeftUp, RightDown, RightUp};
use crate::moves::Square;
use crate::pieces::{Colour, Piece};

#[derive(Clone)]
pub struct Position {
    piece_boards: [[BitBoard; 6]; 2],
    pub player_turn: Colour,
}

impl Position {

    pub fn new_standard_game() -> Self {
        Position {
            piece_boards: [
                [
                    crate::game::STARTING_PAWNS_WHITE,
                    crate::game::STARTING_ROOKS_WHITE,
                    crate::game::STARTING_KNIGHTS_WHITE,
                    crate::game::STARTING_BISHOPS_WHITE,
                    crate::game::STARTING_QUEEN_WHITE,
                    crate::game::STARTING_KING_WHITE,
                ],
                [
                    crate::game::STARTING_PAWNS_BLACK,
                    crate::game::STARTING_ROOKS_BLACK,
                    crate::game::STARTING_KNIGHTS_BLACK,
                    crate::game::STARTING_BISHOPS_BLACK,
                    crate::game::STARTING_QUEEN_BLACK,
                    crate::game::STARTING_KING_BLACK,

                ]
            ],
            player_turn: Colour::White,
        }


    }

    pub fn get_piece(&self, colour: Colour, piece: Piece) -> BitBoard {
        self.piece_boards[colour as usize][piece as usize]
    }

    pub fn make_move(&self, mv: &Move) -> Result<Position, MoveError> {
        let from = mv.from.to_bitboard();
        let to = mv.to.to_bitboard();

        if (&from & &self.get_piece(self.player_turn, mv.piece)).0 == 0 {
            return Err(MoveError::new("No correct colour piece on square"));
        }

        let pseudo_moves = self.get_moves_single_piece(self.player_turn, mv.piece, &mv.from);
        if pseudo_moves.is_none() {
            return Err(MoveError::new("No correct position piece on square"));
        }
        let pseudo_moves = pseudo_moves.unwrap() & !self.get_sides_pieces(self.player_turn);


        if (&to & &pseudo_moves).0 == 0 {
            return Err(MoveError::new("Not a valid destination psuedo move"));
        }
        let index: usize = mv.piece as usize;

        let mut board = self.get_current_turn_boards()[index].clone();
        board = (board ^ from) | to;
        let mut new_pos = self.clone();
        new_pos.update_bitboard(self.player_turn, mv.piece, board);
        new_pos.player_turn = self.player_turn.other();
        let opponent = self.player_turn.other() as usize;
        for i in 0..6 {
            new_pos.piece_boards[opponent][i] = new_pos.piece_boards[opponent][i] & !to;
        }

        let opposition_attacks = new_pos.get_attacks(self.player_turn.other());
        if (opposition_attacks & new_pos.get_piece(self.player_turn, Piece::King)).0 != 0 {
            return Err(MoveError::new("King in check"));
        }
        Ok(new_pos)
    }

    fn get_current_turn_boards(&self) -> [BitBoard; 6] {
        self.piece_boards[self.player_turn as usize]
    }

    pub fn get_sides_pieces(&self, colour: Colour) -> BitBoard {
        let boards = self.piece_boards[colour as usize];
        boards.iter().cloned().reduce(|a,b| a | b).unwrap()
    }

    pub fn get_all_pieces(&self) -> BitBoard {
        self.piece_boards.iter().flatten().copied().reduce(|a,b| a | b).unwrap()
    }

    pub fn get_next_positions(&self) -> Vec<Position> {
        let pieces = self.get_pieces_and_squares(self.player_turn);

        pieces
            .into_iter()
            .map(
                |p| self.get_moves_single_piece(self.player_turn, p.0, &p.1)
                    .map(|moves| self.find_positions_from_move_Set(p.0, &p.1, &moves))
                    .unwrap_or(vec![])
            )
            .flatten()
        .collect()
    }

    pub fn update_bitboard(&mut self, colour: Colour, piece: Piece, value: BitBoard) {
        self.piece_boards[colour as usize][piece as usize] = value;
    }

    fn get_piece_board(&self, colour: Colour, piece: Piece) -> BitBoard {
        self.piece_boards[colour as usize][piece as usize]
    }

    fn find_positions_from_move_Set(&self, piece: Piece, piece_position: &Square, moves: &BitBoard) -> Vec<Position> {

        let mut remaining_moves = moves.clone().0;
        let mut next_moves = Vec::new();
        while remaining_moves != 0 {
            let zeros = remaining_moves.trailing_zeros();
            let to = BitBoard(1 << zeros);
            let piece_board = self.get_piece_board(self.player_turn, piece);
            let piece_position = piece_position.to_bitboard();
            let result = &piece_board & &!piece_position | to.clone();

            let mut next_pos = self.clone();
            next_pos.update_bitboard(self.player_turn, piece, result);


            let opponent = self.player_turn.other() as usize;
            for i in 0..6 {
                next_pos.piece_boards[opponent][i] = next_pos.piece_boards[opponent][i] & !to;
            }
            next_moves.push(next_pos);
            remaining_moves = remaining_moves & !&to.0;
        }
        next_moves
    }

   fn get_pieces_and_squares(&self, colour: Colour) -> Vec<(Piece, Square)> {
       let mut board = self.get_sides_pieces(colour).0;

       let mut squares = Vec::new();
       while board != 0 {
           let sq = board.trailing_zeros();
           let sq = BitBoard(1 << sq);
           let piece = self.piece_boards[colour as usize]
               .iter()
               .enumerate()
               .find(|b| (b.1 & &sq).0 != 0)
               .map(|b| b.0)
               .unwrap();
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

    pub fn get_moves_single_piece(&self, colour: Colour, piece: Piece, square: &Square) -> Option<BitBoard> {
        let board = self.piece_boards[colour as usize][piece as usize];

        let pos = &square.to_bitboard() & &board;
        if pos.0 == 0 {
            return None;
        }
        let pseudo_moves = piece.get_pseudo_moves(&pos.try_into().unwrap(), &self.get_all_pieces(), colour);
        Some(pseudo_moves & ! self.get_sides_pieces(colour))
    }

    pub fn get_moves(&self, colour: Colour, piece: Piece) -> Option<BitBoard> {
        let squares = self.find_piece_squares(piece, colour);

        squares.into_iter()
            .map(|sq| self.get_moves_single_piece(colour, piece, &sq))
            .flatten()
            .reduce(|a,b| a | b)
    }

    pub fn find_piece_squares(&self, piece: Piece, colour: Colour) -> Vec<Square> {
        let mut board: u64 = self.get_piece(colour, piece).0;

        let mut result = vec![];
        while board != 0 {
            let lowest_occupied = board.trailing_zeros();
            let square = 1 << lowest_occupied;
            let square = BitBoard(square);

            result.push(Square::try_from(square).unwrap());
            board &= board - 1;
        }

        result
    }

    pub fn get_attacks(&self, colour: Colour) -> BitBoard {
        let attacks = self.get_moves(colour, Piece::Knight).unwrap_or(BitBoard(0))
            | self.get_moves(colour, Piece::Bishop).unwrap_or(BitBoard(0))
            | self.get_moves(colour, Piece::Rook).unwrap_or(BitBoard(0))
            | self.get_moves(colour, Piece::Queen).unwrap_or(BitBoard(0))
            | self.get_moves(colour, Piece::King).unwrap_or(BitBoard(0));

        attacks | self.get_pawn_attacks(colour)
    }

    pub fn get_pawn_attacks(&self, colour: Colour) -> BitBoard {
        let board = self.piece_boards[colour as usize][Piece::Pawn as usize];

        match colour {
            Colour::White => board.shift(LeftUp) | board.shift(RightUp),
            Colour::Black => board.shift(LeftDown) | board.shift(RightDown),
        }
    }
}

