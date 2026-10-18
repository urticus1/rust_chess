use crate::game::{Move, NormalMove};
use crate::pieces::Piece;
use crate::position::Position;

pub fn generate_valid_moves(position: &Position) -> Vec<Move> {
    let mut result: Vec<Move> = Vec::new();
    for piece in Piece::ALL {
        let board = position.get_piece(position.player_turn, *piece);

        for sq in board {
            let mvs = position.get_pseudo_moves_single_piece(position.player_turn, *piece, sq);
            if let Some(pseudo_moves) = mvs {
                let pseudo_moves = pseudo_moves & !position.get_sides_pieces(position.player_turn);
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
    }
    result
}