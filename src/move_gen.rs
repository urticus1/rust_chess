use crate::game::{CastleSide, Move, NormalMove};
use crate::pieces::{bishop_moves_pext, rook_moves_pext, Colour, Piece, KNIGHT_MOVES};
use crate::position::Position;
use crate::squares::{Direction, Square};

pub fn generate_valid_moves(position: &mut Position) -> [Option<Move>; 256] {
    let mut result = [None; 256];
    let mut i = 0;
    for piece in Piece::ALL {
        let board = position.get_piece(position.player_turn, *piece);

        for from_sq in board {
            let mvs = position.get_pseudo_moves_single_piece(position.player_turn, *piece, from_sq);
            let pseudo_moves = mvs & !position.get_sides_pieces(position.player_turn);
            for to_sq in pseudo_moves {
                let mv = Move::Normal(NormalMove {
                    piece: *piece,
                    from: from_sq,
                    to: to_sq,
                });
                let anti_move = position.make_reversible_move(mv);
                if !is_check(position) {
                    result[i] = Some(mv);
                    i = i + 1;
                }
                position.reverse_move(anti_move);
            }
        }
    }
    if position.can_castle(position.player_turn, CastleSide::King) {
        //result.push(Castle(CastleSide::King, position.player_turn));
    }
    if position.can_castle(position.player_turn, CastleSide::Queen) {
        //result.push(Castle(CastleSide::Queen, position.player_turn));
    }
    result
}

fn is_check(position: &Position) -> bool {
    let king = position.get_piece(position.player_turn.other(), Piece::King);
    let king_square = Square::try_from(king).unwrap();
    let blockers = position.get_all_pieces();
    if bishop_moves_pext(king_square, blockers).intersects(position.get_piece(position.player_turn, Piece::Bishop)) {
        return true;
    }
    if bishop_moves_pext(king_square, blockers).intersects(position.get_piece(position.player_turn, Piece::Queen)) {
        return true;
    }
    if rook_moves_pext(king_square, blockers).intersects(position.get_piece(position.player_turn, Piece::Rook)) {
        return true;
    }
    if rook_moves_pext(king_square, blockers).intersects(position.get_piece(position.player_turn, Piece::Queen)) {
        return true;
    }
    if KNIGHT_MOVES[king_square as usize].intersects(position.get_piece(position.player_turn, Piece::Knight)) {
        return true;
    }
    match position.player_turn.other() {
        Colour::White => {
            if (king.shift(Direction::LeftUp) | king.shift(Direction::RightUp)).intersects(position.get_piece(position.player_turn, Piece::Pawn)) {
                return true;
            }
        },
        Colour::Black => {
            if (king.shift(Direction::LeftDown) | king.shift(Direction::RightDown)).intersects(position.get_piece(position.player_turn, Piece::Pawn)) {
                return true;
            }
        }
    }
    false
}