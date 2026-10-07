use std::arch::x86_64::_mm_minpos_epu16;
use crate::pieces::{king_moves, Colour, Piece};
use crate::position::Position;

const PAWN_VALUE: f32 = 1.0;
const KNIGHT_VALUE: f32 = 3.0;
const BISHOP_VALUE: f32 = 3.0;
const ROOK_VALUE: f32 = 5.0;
const QUEEN_VALUE: f32 = 9.0;

pub fn eval(position: &Position) -> f32 {
    material_score(position, Colour::White) - material_score(position, Colour::Black)
}

enum MinMax {
    Min,
    Max
}

pub fn get_best_move(position: &Position) -> (f32, Position, Position) {
    let start_pos = position.clone();
    search(0, &start_pos)
}

fn search(depth: u8, position: &Position) -> (f32, Position, Position) {
    if depth >= 4 {
        return (eval(position), position.clone(), position.clone());
    }

    let mut best_score = match position.player_turn {
        Colour::White => -f32::INFINITY,
        Colour::Black => f32::INFINITY
    };
    let mut best_move = position.clone();
    let mut final_pos= position.clone();

    for child in position.get_next_positions() {
        let (eval, mv, fp) = search(depth + 1, &child);
        match position.player_turn {
            Colour::White => {
                if eval >= best_score {
                    best_score = eval;
                    best_move = child;
                    final_pos = fp
                }
            },
            Colour::Black => {
                if eval <= best_score {
                    best_score = eval;
                    best_move = child;
                    final_pos = fp
                }
            }
        }
    }
    (best_score, best_move, final_pos)
}

fn material_score(position: &Position, colour: Colour) -> f32 {
    let pawns = position.get_piece(colour, Piece::Pawn).count_ones() as f32;
    let knights = position.get_piece(colour, Piece::Knight).count_ones() as f32;
    let bishops = position.get_piece(colour, Piece::Bishop).count_ones() as f32;
    let rooks = position.get_piece(colour, Piece::Rook).count_ones() as f32;
    let queens = position.get_piece(colour, Piece::Queen).count_ones() as f32;

      pawns * PAWN_VALUE
    + knights * KNIGHT_VALUE
    + bishops * BISHOP_VALUE
    + rooks * ROOK_VALUE
    + queens * QUEEN_VALUE
}

fn is_checkmate(position: &Position) -> bool {
    let enemy_attacks = position.get_attacks(position.player_turn.other());
    let blockers = position.get_sides_pieces(position.player_turn);
    let king = position.get_piece(position.player_turn, Piece::King);
    let king_moves = king_moves(king);
    enemy_attacks.intersects(king) && (king_moves & !blockers & !enemy_attacks).is_empty()
}

fn is_check(position: &Position) -> bool {
    let king = position.get_piece(position.player_turn, Piece::King);
    let enemy_attacks = position.get_attacks(position.player_turn.other());
    enemy_attacks.intersects(king)
}