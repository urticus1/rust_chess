use crate::game::{CastleSide, Move, NormalMove};
use crate::game::Move::Castle;
use crate::pieces::{bishop_moves_pext, rook_moves_pext, Colour, Piece, KING_MOVES, KNIGHT_MOVES};
use crate::position::Position;
use crate::squares::{Direction, Square};

pub fn generate_valid_moves(position: &mut Position) -> [Option<Move>; 256] {
    let mut result = [None; 256];
    let mut i = 0;
    let mut captures_index = 0;
    let opposition_pieces = position.get_sides_pieces(position.player_turn.other());
    for piece in Piece::ALL {
        let board = position.get_piece(position.player_turn, *piece);

        for from_sq in board {
            let mvs = position.get_pseudo_moves_single_piece(position.player_turn, *piece, from_sq);
            let pseudo_moves = mvs & !position.get_sides_pieces(position.player_turn);
            for to_sq in pseudo_moves {
                let to_bitboard = to_sq.to_bitboard();
                if to_bitboard.intersects(opposition_pieces) {
                    let mv = Move::Capture(NormalMove {
                        piece: *piece,
                        from: from_sq,
                        to: to_sq,
                    });
                    result[i] = result[captures_index];
                    result[captures_index] = Some(mv);
                    i+= 1;
                    captures_index+=1;

                }
                else {
                    let mv = Move::Normal(NormalMove {
                        piece: *piece,
                        from: from_sq,
                        to: to_sq,
                    });
                    result[i] = Some(mv);
                    i = i + 1;
                };
            }
        }
    }

    if position.can_castle(position.player_turn, CastleSide::King) {
        result[i] = Some(Castle(CastleSide::King, position.player_turn));
        i += 1;
    }
    if position.can_castle(position.player_turn, CastleSide::Queen) {
        result[i] = Some(Castle(CastleSide::Queen, position.player_turn));
    }
    result
}

pub fn is_square_attacked_by_colour(position: &Position, sq: Square, colour: Colour) -> bool {
    let blockers = position.get_all_pieces();
    if bishop_moves_pext(sq, blockers).intersects(position.get_piece(colour, Piece::Bishop)) {
        return true;
    }
    if bishop_moves_pext(sq, blockers).intersects(position.get_piece(colour, Piece::Queen)) {
        return true;
    }
    if rook_moves_pext(sq, blockers).intersects(position.get_piece(colour, Piece::Rook)) {
        return true;
    }
    if rook_moves_pext(sq, blockers).intersects(position.get_piece(colour, Piece::Queen)) {
        return true;
    }
    if KNIGHT_MOVES[sq as usize].intersects(position.get_piece(colour, Piece::Knight)) {
        return true;
    }
    if KING_MOVES[sq as usize].intersects(position.get_piece(colour, Piece::King)) {
        return true;
    }

    let board = sq.to_bitboard();
    match colour {
        Colour::White => {
            if (board.shift(Direction::LeftDown) | board.shift(Direction::RightDown)).intersects(position.get_piece(colour, Piece::Pawn)) {
                return true;
            }
        },
        Colour::Black => {
            if (board.shift(Direction::LeftUp) | board.shift(Direction::RightUp)).intersects(position.get_piece(colour, Piece::Pawn)) {
                return true;
            }
        }
    }
    false
}