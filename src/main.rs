mod pieces;
mod game;
mod bitboard;
mod squares;
mod position;
mod engine;
mod move_gen;

use std::io;
use std::ops::{BitAnd, BitOr, Not};
use crate::bitboard::{BitBoard};
use crate::engine::get_best_move;
use crate::game::{CastleSide, GameState, Move, NormalMove};
use crate::squares::{Square};
use crate::pieces::{generate_all_rook_move_sets, Colour, Piece};
use crate::squares::Square::{A1, A7, C3, G6};

const ROW_MASK: u64 = 0b_11111111;

const ROW_1: BitBoard = BitBoard(ROW_MASK);
const ROW_2: BitBoard = BitBoard(ROW_MASK << 8);
const ROW_3: BitBoard = BitBoard(ROW_MASK << 16);
const ROW_4: BitBoard = BitBoard(ROW_MASK << 24);
const ROW_5: BitBoard = BitBoard(ROW_MASK << 32);
const ROW_6: BitBoard = BitBoard(ROW_MASK << 40);
const ROW_7: BitBoard = BitBoard(ROW_MASK << 48);
const ROW_8: BitBoard = BitBoard(ROW_MASK << 56);

const COL_A: BitBoard = BitBoard(0b_0000000100000001000000010000000100000001000000010000000100000001 );

const COL_B: BitBoard = BitBoard(0b_0000001000000010000000100000001000000010000000100000001000000010 );
const COL_C: BitBoard = BitBoard(0b_0000010000000100000001000000010000000100000001000000010000000100 );
const COL_D: BitBoard = BitBoard( 0b_0000100000001000000010000000100000001000000010000000100000001000 );
const COL_E:  BitBoard = BitBoard(0b_0001000000010000000100000001000000010000000100000001000000010000 );
const COL_F: BitBoard = BitBoard(0b_0010000000100000001000000010000000100000001000000010000000100000 );
const COL_G: BitBoard = BitBoard(0b_0100000001000000010000000100000001000000010000000100000001000000 );
const COL_H:  BitBoard = BitBoard(0b_1000000010000000100000001000000010000000100000001000000010000000 );

const INTERIOR_SQUARES:  BitBoard = BitBoard(!(ROW_1.0 | ROW_8.0 | COL_A.0 | COL_H.0));

fn main() {

    let mut game = GameState::new_standard_game();

    loop {
        let mut buffer = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut buffer).unwrap();

        let castle_side = match buffer.trim().to_uppercase().as_str() {
            "O-O" | "0-0" => Some(CastleSide::King),
            "O-O-O" | "0-0-0" => Some(CastleSide::Queen),
            _ => None,
        };

        let mv = if let Some(side) = castle_side {
            Move::Castle(side, game.pos.player_turn)
        } else {
            let mut buffer = buffer.chars();

            let piece_choice = buffer.next();
            let piece =
                match piece_choice{
                    Some(c) => {
                        match c.to_uppercase().next().unwrap() {
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

            let mv = buffer.as_str();
            let from = mv[0..2].to_string();
            let to = mv[2..4].to_string();

            println!("{}", from);
            println!("{}", to);
            println!("{:?}", Square::try_from(from.clone()).unwrap());

            Move::Normal (
                NormalMove {
                    piece,
                    from: Square::try_from(from).unwrap(),
                    to: Square::try_from(to).unwrap()
                }
            )
        };

        game.make_move(mv);
        let computer_move = get_best_move(&game.pos);
        println!("Computer eval: {}", computer_move.0);
        println!("Computer mv: {:?}", computer_move.1);
        if let Some(computer_move) = computer_move.1 {
            game.make_move(computer_move);
        }
        println!("{}", game.pos);
    }
}



