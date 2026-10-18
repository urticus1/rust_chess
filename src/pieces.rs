use crate::bitboard::{BitBoard};
use crate::{ROW_2, ROW_7};
use crate::pieces::Piece::{Bishop, King, Knight, Pawn, Queen, Rook};
use crate::squares::{Direction, Square};
use crate::position::Position;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Piece {
    Pawn = 0,
    Rook = 1,
    Knight = 2,
    Bishop = 3,
    Queen = 4,
    King = 5
}

impl Piece {
    pub const ALL: &'static [Piece; 6] = &[Pawn, Rook, Knight, Bishop, Queen, King];
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Colour {
    White = 0,
    Black = 1,
}

impl Colour {
    pub fn other(&self) -> Colour {
        match self {
            Colour::White => Colour::Black,
            Colour::Black => Colour::White,
        }
    }
}

impl Piece {
    pub fn get_pseudo_moves(&self, position: &Position, piece_pos: &Square, pieces: BitBoard, colour: Colour) -> BitBoard {

        match self {
            Piece::Pawn => single_pawn_moves(piece_pos.to_bitboard(), position, colour) | (single_pawn_pseudo_attacks(piece_pos.to_bitboard(), colour) & position.get_sides_pieces(colour.other())),
            Piece::Knight => knight_moves(piece_pos.to_bitboard()),
            Piece::Bishop => bishop_moves(piece_pos, pieces),
            Piece::Rook => rook_moves(piece_pos, pieces),
            Piece::Queen => queen_moves(piece_pos, pieces),
            Piece::King => king_moves(piece_pos.to_bitboard()),
        }
    }
}

fn single_pawn_moves(pawn: BitBoard, pos: &Position, colour: Colour) -> BitBoard {
    let blockers = pos.get_all_pieces();
    match colour {
        Colour::White => {
            let first_move = (pawn & ROW_2).shift(Direction::Up) & !blockers;
            let remaining_moves = (first_move | pawn).shift(Direction::Up) & !blockers;
            first_move | remaining_moves
        },
        Colour::Black => {
            let first_move = (pawn & ROW_7).shift(Direction::Down) & !blockers;
            let remaining_moves = (first_move | pawn).shift(Direction::Down) & !blockers;
            first_move | remaining_moves
        }
    }
}

fn single_pawn_pseudo_attacks(pawn: BitBoard, colour: Colour) -> BitBoard {
    match colour {
        Colour::White => pawn.shift(Direction::LeftUp) | pawn.shift(Direction::RightUp),
        Colour::Black => pawn.shift(Direction::LeftDown) | pawn.shift(Direction::RightDown),
    }
}
pub fn all_pawn_attacks(pos: &Position, colour: Colour) -> BitBoard {
    let board = pos.get_piece_board(colour, Piece::Pawn);

    match colour {
        Colour::White => board.shift(Direction::LeftUp) | board.shift(Direction::RightUp),
        Colour::Black => board.shift(Direction::LeftDown) | board.shift(Direction::RightDown),
    }
}

//TODO add castling in here?
pub fn king_moves(pos: BitBoard) -> BitBoard {
    pos.shift(Direction::Up)
    | pos.shift(Direction::Down)
    | pos.shift(Direction::Left)
    | pos.shift(Direction::Right)
    | pos.shift(Direction::LeftUp)
    | pos.shift(Direction::RightDown)
    | pos.shift(Direction::LeftDown)
    | pos.shift(Direction::RightUp)
}

fn knight_moves(pos: BitBoard) -> BitBoard {
    pos.shift(Direction::Up).shift(Direction::Up).shift(Direction::Right)
        | pos.shift(Direction::Up).shift(Direction::Up).shift(Direction::Left)
        | pos.shift(Direction::Down).shift(Direction::Down).shift(Direction::Right)
        | pos.shift(Direction::Down).shift(Direction::Down).shift(Direction::Left)

        | pos.shift(Direction::Left).shift(Direction::Left).shift(Direction::Up)
        | pos.shift(Direction::Left).shift(Direction::Left).shift(Direction::Down)
        | pos.shift(Direction::Right).shift(Direction::Right).shift(Direction::Up)
        | pos.shift(Direction::Right).shift(Direction::Right).shift(Direction::Down)
}

pub fn rook_moves(pos: &Square, pieces: BitBoard) -> BitBoard {
    let mut result = BitBoard (0 );
    for dir in [Direction::Up, Direction::Down, Direction::Right, Direction::Left] {
        let mut next = pos.get_neighbour(&dir);
        while next.is_some() {
            let current = next.unwrap();
            let pos_bitboard = current.to_bitboard();
            result = &result | &pos_bitboard;
            next = current.get_neighbour(&dir);
            if (pieces & pos_bitboard).0 != 0 {
                break;
            }
        }
    }
    result
}

pub fn queen_moves(pos: &Square, pieces: BitBoard) -> BitBoard {
    let mut result = BitBoard (0 );
    for dir in [Direction::LeftUp, Direction::LeftDown, Direction::RightUp, Direction::RightDown, Direction::Down, Direction::Up, Direction::Left, Direction::Right] {
        let mut next = pos.get_neighbour(&dir);
        while next.is_some() {
            let current = next.unwrap();
            let pos_bitboard = current.to_bitboard();
            result = &result | &pos_bitboard;
            next = current.get_neighbour(&dir);
            if (pieces & pos_bitboard).0 != 0 {
                break;
            }
        }
    }
    result
}

pub fn bishop_moves(pos: &Square, pieces: BitBoard) -> BitBoard {
    let mut result = BitBoard (0 );
    for dir in [Direction::LeftUp, Direction::LeftDown, Direction::RightUp, Direction::RightDown] {
        let mut next = pos.get_neighbour(&dir);
        while next.is_some() {
            let current = next.unwrap();
            let pos_bitboard = current.to_bitboard();
            result = &result | &pos_bitboard;
            next = current.get_neighbour(&dir);
            if (pieces & pos_bitboard).0 != 0 {
                break;
            }
        }
    }
    result
}


#[cfg(test)]
mod tests {
    use crate::pieces::{Colour, Piece};
    use crate::position::Position;
    use crate::squares::Square;

    #[test]
    fn test_rook_captures() {

        let mut pos = Position::blank();
        pos.update_bitboard(Colour::White, Piece::Rook, Square::A1.to_bitboard());

        pos.update_bitboard(Colour::White, Piece::Queen, Square::A2.to_bitboard());
        pos.update_bitboard(Colour::Black, Piece::Queen, Square::B1.to_bitboard());

        let moves = pos.get_moves(Colour::White, Piece::Rook);

        assert!(moves.is_some());
        assert!(Square::B1.to_bitboard() == moves.unwrap())
    }
}