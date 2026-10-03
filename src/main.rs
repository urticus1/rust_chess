mod pieces;
mod game;
mod bitboard;
mod moves;
mod position;
mod engine;

use std::io;
use std::ops::{BitAnd, BitOr, Not};
use crate::bitboard::{BitBoard};
use crate::game::{GameState, Move};
use crate::moves::{Column, Row, Square};
use crate::pieces::Colour::{Black, White};
use crate::pieces::{queen_moves, rook_moves, Piece};
use crate::position::Position;

const ROW_MASK: u64 = 0b_11111111;

const ROW_1: BitBoard = BitBoard { board: ROW_MASK };
const ROW_2: BitBoard = BitBoard { board: ROW_MASK << 8 };
const ROW_3: BitBoard = BitBoard { board: ROW_MASK << 16 };
const ROW_4: BitBoard = BitBoard { board: ROW_MASK << 24 };
const ROW_5: BitBoard = BitBoard { board: ROW_MASK << 32 };
const ROW_6: BitBoard = BitBoard { board: ROW_MASK << 40 };
const ROW_7: BitBoard = BitBoard { board: ROW_MASK << 48 };
const ROW_8: BitBoard = BitBoard { board: ROW_MASK << 56 };



const COL_A: BitBoard = BitBoard { board: 0b_0000000100000001000000010000000100000001000000010000000100000001 };

const COL_B: BitBoard = BitBoard { board: 0b_0000001000000010000000100000001000000010000000100000001000000010 };
const COL_C: BitBoard = BitBoard { board: 0b_0000010000000100000001000000010000000100000001000000010000000100 };
const COL_D: BitBoard = BitBoard { board:  0b_0000100000001000000010000000100000001000000010000000100000001000 };
const COL_E:  BitBoard = BitBoard { board: 0b_0001000000010000000100000001000000010000000100000001000000010000 };
const COL_F: BitBoard = BitBoard { board: 0b_0010000000100000001000000010000000100000001000000010000000100000 };
const COL_G: BitBoard = BitBoard { board: 0b_0100000001000000010000000100000001000000010000000100000001000000 };
const COL_H:  BitBoard = BitBoard { board:  0b_1000000010000000100000001000000010000000100000001000000010000000 };

fn main() {

    let mut game = GameState::new_standard_game();

    let pos = Position::new_standard_game();
    println!("{}", pos.get_next_positions().len());

    game.get_sides_pieces(&White).print();
    game.get_sides_pieces(&Black).print();

    loop {
        let mut buffer = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut buffer).unwrap();

        let mut buffer = buffer.chars();

        let piece_choice = buffer.next();
        let piece =
            match piece_choice{
                Some(c) => {
                    match c {
                        'N' => Piece::Knight,
                        'B' => Piece::Bishop,
                        'Q' => Piece::Queen,
                        'R' => Piece::Rook,
                        'K' => Piece::King,
                        'P' => Piece::Pawn,
                        _ => panic!("Invalid piece name")
                    }
                },
                None => panic!("Invalid piece name")
            };

        let from_column = match buffer.next() {
            Some(c) => match c {
                'a' => Column::A,
                'b' => Column::B,
                'c' => Column::C,
                'd' => Column::D,
                'e' => Column::E,
                'f' => Column::F,
                'g' => Column::G,
                'h' => Column::H,
                _ => panic!("Invalid column name"),
            },
            None => panic!("Invalid column name"),
        };

        let from_row = match buffer.next() {
            Some(c) => match c {
                '1' => Row::One,
                '2' => Row::Two,
                '3' => Row::Three,
                '4' => Row::Four,
                '5' => Row::Five,
                '6' => Row::Six,
                '7' => Row::Seven,
                '8' => Row::Eight,
                _ => panic!("Invalid row name"),
            },
            None => panic!("Invalid row name"),
        };

        let to_column = match buffer.next() {
            Some(c) => match c {
                'a' => Column::A,
                'b' => Column::B,
                'c' => Column::C,
                'd' => Column::D,
                'e' => Column::E,
                'f' => Column::F,
                'g' => Column::G,
                'h' => Column::H,
                _ => panic!("Invalid column name"),
            },
            None => panic!("Invalid column name"),
        };

        let to_row = match buffer.next() {
            Some(c) => match c {
                '1' => Row::One,
                '2' => Row::Two,
                '3' => Row::Three,
                '4' => Row::Four,
                '5' => Row::Five,
                '6' => Row::Six,
                '7' => Row::Seven,
                '8' => Row::Eight,
                _ => panic!("Invalid row name"),
            },
            None => panic!("Invalid row name"),
        };
        let mv = Move {
            piece,
            from: Square { column: from_column, row: from_row },
            to: Square { column: to_column, row: to_row },
        };

        match game.make_move(&mv) {
            Ok(()) => game.get_all_pieces().print(),
            Err(e) => println!("{}", e)
        }

    }
}



