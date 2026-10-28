mod pieces;
mod game;
mod bitboard;
mod squares;
mod position;
mod engine;
mod move_gen;

use std::io;
use std::time::Instant;
use std::ops::{BitAnd, BitOr, Not};
use crate::engine::get_best_move;
use crate::game::{CastleSide, GameState, Move, NormalMove};
use crate::squares::{Square};
use crate::pieces::{Piece};
use crate::position::Position;


fn main() {

    if std::env::args().nth(1).as_deref() == Some("bench") {
        run_bench();
        return;
    }

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

fn run_bench() {
    let pos = Position::new_standard_game();
    let start = Instant::now();
    let (eval, mv) = get_best_move(&pos);
    println!("search from start: {:.2?}  eval={} mv={:?}", start.elapsed(), eval, mv);
}



