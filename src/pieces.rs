use crate::bitboard::{BitBoard};
use crate::{ROW_2, ROW_7};
use crate::moves::{Direction, Square};

pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Clone)]
pub enum Colour {
    White,
    Black,
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
    pub fn get_pseudo_moves(&self, pos: &Square, pieces: &BitBoard, colour: &Colour) -> BitBoard {

        match self {
            Piece::Pawn => pawn_moves(&pos.to_bitboard(), colour),
            Piece::Knight => knight_moves(&pos.to_bitboard()),
            Piece::Bishop => bishop_moves(pos, pieces),
            Piece::Rook => rook_moves(pos, pieces),
            Piece::Queen => queen_moves(pos, pieces),
            Piece::King => king_moves(&pos.to_bitboard()),
        }
    }
}

fn pawn_moves(pos: &BitBoard, colour: &Colour) -> BitBoard {
    match colour {
        Colour::White => {
            (pos.clone() & ROW_2).shift(Direction::Up).shift(Direction::Up)
                | pos.shift(Direction::Up)
        },
        Colour::Black => {
            (pos.clone() & ROW_7).shift(Direction::Down).shift(Direction::Down)
                | pos.shift(Direction::Down)
        }
    }
}

fn king_moves(pos: &BitBoard) -> BitBoard {
    pos.shift(Direction::Up)
    | pos.shift(Direction::Down)
    | pos.shift(Direction::Left)
    | pos.shift(Direction::Right)
    | pos.shift(Direction::LeftUp)
    | pos.shift(Direction::RightDown)
    | pos.shift(Direction::LeftDown)
    | pos.shift(Direction::RightUp)
}

fn knight_moves(pos: &BitBoard) -> BitBoard {
    pos.shift(Direction::Up).shift(Direction::Up).shift(Direction::Right)
        | pos.shift(Direction::Up).shift(Direction::Up).shift(Direction::Left)
        | pos.shift(Direction::Down).shift(Direction::Down).shift(Direction::Right)
        | pos.shift(Direction::Down).shift(Direction::Down).shift(Direction::Left)

        | pos.shift(Direction::Left).shift(Direction::Left).shift(Direction::Up)
        | pos.shift(Direction::Left).shift(Direction::Left).shift(Direction::Down)
        | pos.shift(Direction::Right).shift(Direction::Right).shift(Direction::Up)
        | pos.shift(Direction::Right).shift(Direction::Right).shift(Direction::Down)
}

pub fn rook_moves(pos: &Square, pieces: &BitBoard) -> BitBoard {
    let mut result = BitBoard (0 );
    for dir in [Direction::Up, Direction::Down, Direction::Right, Direction::Left] {
        let mut next = pos.get_neighbour(&dir);
        while next.is_some() {
            let current = next.unwrap();
            let pos_bitboard = current.to_bitboard();
            result = &result | &pos_bitboard;
            next = current.get_neighbour(&dir);
            if (pieces & &pos_bitboard).0 != 0 {
                break;
            }
        }
    }
    result
}

pub fn queen_moves(pos: &Square, pieces: &BitBoard) -> BitBoard {
    let mut result = BitBoard (0 );
    for dir in [Direction::LeftUp, Direction::LeftDown, Direction::RightUp, Direction::RightDown, Direction::Down, Direction::Up, Direction::Left, Direction::Right] {
        let mut next = pos.get_neighbour(&dir);
        while next.is_some() {
            let current = next.unwrap();
            let pos_bitboard = current.to_bitboard();
            result = &result | &pos_bitboard;
            next = current.get_neighbour(&dir);
            if (pieces & &pos_bitboard).0 != 0 {
                break;
            }
        }
    }
    result
}

pub fn bishop_moves(pos: &Square, pieces: &BitBoard) -> BitBoard {
    let mut result = BitBoard (0 );
    for dir in [Direction::LeftUp, Direction::LeftDown, Direction::RightUp, Direction::RightDown] {
        let mut next = pos.get_neighbour(&dir);
        while next.is_some() {
            let current = next.unwrap();
            let pos_bitboard = current.to_bitboard();
            result = &result | &pos_bitboard;
            next = current.get_neighbour(&dir);
            if (pieces & &pos_bitboard).0 != 0 {
                break;
            }
        }
    }
    result
}

impl Piece {
    pub fn to_bitboard_index(&self) -> usize {
        match self {
            Piece::Pawn => 0,
            Piece::Rook => 1,
            Piece::Knight => 2,
            Piece::Bishop => 3,
            Piece::Queen => 4,
            Piece::King => 5,
        }
    }
}
impl Into<usize> for Piece {
    fn into(self) -> usize {
        match self {
            Piece::Pawn => 0,
            Piece::Rook => 1,
            Piece::Knight => 2,
            Piece::Bishop => 3,
            Piece::Queen => 4,
            Piece::King => 5,
        }
    }
}

impl Into<usize> for &Piece {
    fn into(self) -> usize {
        match self {
            Piece::Pawn => 0,
            Piece::Rook => 1,
            Piece::Knight => 2,
            Piece::Bishop => 3,
            Piece::Queen => 4,
            Piece::King => 5,
        }
    }
}
