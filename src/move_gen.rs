use crate::game::{CastleSide, Move, NormalMove};
use crate::game::Move::Castle;
use crate::pieces::Piece;
use crate::position::Position;

pub fn generate_valid_moves(position: &Position) -> Vec<Move> {
    let mut result: Vec<Move> = Vec::new();
    for piece in Piece::ALL {
        let board = position.get_piece(position.player_turn, *piece);

        for sq in board {
            let mvs = position.get_pseudo_moves_single_piece(position.player_turn, *piece, sq);
            let pseudo_moves = mvs & !position.get_sides_pieces(position.player_turn);
            for mv in pseudo_moves {
                result.push(Move::Normal(
                    NormalMove {
                        piece: *piece,
                        from: sq,
                        to: mv,
                    }
                ))
            }
        }

    }
    if position.can_castle(position.player_turn, CastleSide::King) {
        result.push(Castle(CastleSide::King, position.player_turn));
    }
    if position.can_castle(position.player_turn, CastleSide::Queen) {
        result.push(Castle(CastleSide::Queen, position.player_turn));
    }
    result
}