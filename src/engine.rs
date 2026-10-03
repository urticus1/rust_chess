use crate::pieces::{Colour, Piece};
use crate::position::Position;

const PAWN_VALUE: f32 = 1.0;
const KNIGHT_VALUE: f32 = 3.0;
const BISHOP_VALUE: f32 = 3.0;
const ROOK_VALUE: f32 = 5.0;
const QUEEN_VALUE: f32 = 9.0;


pub fn eval(position: &Position) -> f32 {
  material_score(position, &Colour::White) - material_score(position, &Colour::Black)
}

fn material_score(position: &Position, colour: &Colour) -> f32 {
    let pawns = position.get_piece(colour, &Piece::Pawn).board.count_ones() as f32;
    let knights = position.get_piece(colour, &Piece::Knight).board.count_ones() as f32;
    let bishops = position.get_piece(colour, &Piece::Bishop).board.count_ones() as f32;
    let rooks = position.get_piece(colour, &Piece::Rook).board.count_ones() as f32;
    let queens = position.get_piece(colour, &Piece::Queen).board.count_ones() as f32;

      pawns * PAWN_VALUE
    + knights * KNIGHT_VALUE
    + bishops * BISHOP_VALUE
    + rooks * ROOK_VALUE
    + queens * QUEEN_VALUE
}

fn best_move(position: &Position) -> Position {
    for mv in position.get_legal_moves() {

    }
    todo!()
}