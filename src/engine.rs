use std::f32::INFINITY;
use crate::game::Move;
use crate::move_gen::generate_valid_moves;
use crate::pieces::{bishop_moves_pext, rook_moves_pext, Colour, Piece, KNIGHT_MOVES};
use crate::position::Position;
use crate::squares::{Direction, Square};

const PAWN_VALUE: f32 = 1.0;
const KNIGHT_VALUE: f32 = 3.0;
const BISHOP_VALUE: f32 = 3.0;
const ROOK_VALUE: f32 = 5.0;
const QUEEN_VALUE: f32 = 9.0;


macro_rules! heat_map {
      ($name:ident { $( [$($sq:expr),* ]),* $(,)?}) => {
          const $name: [f32; 64] = [$( $($sq,)* )*];
      };
}

heat_map!( KNIGHT_MAP {
    [-0.3, -0.3, -0.3, -0.3, -0.3, -0.3, -0.3, -0.3],
    [-0.3, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.3],
    [-0.3, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.3],
    [-0.3, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.3],
    [-0.3, 0.2, 0.2, 0.2, 0.2, 0.2, 0.2, -0.3],
    [-0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, -0.3],
    [-0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, -0.3],
    [-0.3, -0.3, -0.3, -0.3, -0.3, -0.3, -0.3, -0.3],
});

heat_map!( PAWN_MAP {
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.1, 0.1, 0.1, 0.1, 0.0, 0.0],
    [0.0, 0.0, 0.2, 0.2, 0.2, 0.2, 0.0, 0.0],
    [0.0, 0.0, 0.3, 0.3, 0.3, 0.3, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
});

heat_map!( BLANK_MAP {
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
});

heat_map!( KING_MAP {
    [0.0, 0.5, 0.5, 0.0, 0.0, 0.0, 0.5, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
});


pub fn eval(position: &Position) -> f32 {

    let mut positional_score = 0.0;
    for player in [Colour::White, Colour::Black] {
        positional_score += Piece::ALL.iter()
            .flat_map(|piece| position.get_piece(player, *piece).map(move |sq| (sq, piece)))
            .map(|(sq, piece)| {
                let heat_map = match piece {
                    Piece::Pawn => &PAWN_MAP,
                    Piece::Knight => &KNIGHT_MAP,
                    Piece::King => &KING_MAP,
                    _ => &BLANK_MAP
                };
                let index = if player == Colour::White { sq as usize } else { 63 - sq as usize };
                heat_map[index]
            })
            .map(|score| {
                if player == Colour::Black {
                    return score * -1.0;
                }
                score
            })
        .sum::<f32>();
    }

    material_score(position, Colour::White) - material_score(position, Colour::Black) + positional_score
}

pub fn get_best_move(position: &Position) -> (f32, Option<Move>) {
    let node = search(6, position, -f32::INFINITY, f32::INFINITY);
    (node.eval, node.mv)
}

fn search(depth: u8, position: &Position, mut alpha: f32, mut beta: f32) -> Node {
    if depth == 0 {
        return Node {
            mv: None,
            eval: eval(position),
        }
    }

    let mut best_score = match position.player_turn {
        Colour::White => -f32::INFINITY,
        Colour::Black => f32::INFINITY
    };
    let mut best_move = None;

    let next_moves = generate_valid_moves(position);

    if next_moves.is_empty() {
        if is_check(position) && is_checkmate(position) {
            return match position.player_turn {
                Colour::White => {
                    Node {
                        mv: best_move,
                        eval: f32::INFINITY,

                    }
                },
                Colour::Black => {
                    Node {
                        mv: best_move,
                        eval: f32::NEG_INFINITY,

                    }
                }
            }
        }
        else {
            return Node {
                mv: best_move,
                eval: 0.0,
            }
        }
    }

    for mv in next_moves {
        let new_pos = position.make_move(mv);
        if let Ok(valid_pos) = new_pos {
            let node = search(depth - 1, &valid_pos, alpha, beta);
            match position.player_turn {
                Colour::White => {
                    alpha = alpha.max(node.eval);
                    if node.eval > best_score {
                        best_score = node.eval;
                        best_move = Some(mv);
                    }
                    if alpha >= beta {
                        break;
                    }

                },
                Colour::Black => {

                    beta = beta.min(node.eval);
                    if node.eval < best_score {
                        best_score = node.eval;
                        best_move = Some(mv);
                    }
                    if beta <= alpha {
                        break;
                    }
                }
            }
        }
    }

    Node {
        mv: best_move,
        eval: best_score,

    }
}

struct Node {
    mv: Option<Move>,
    eval: f32,

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

fn is_checkmate(position: &Position) -> bool { //TODO finish this
    let enemy_attacks = position.get_attacks(position.player_turn.other());
    let blockers = position.get_sides_pieces(position.player_turn);
    let king = position.get_piece(position.player_turn, Piece::King);
    //let king_moves = king_moves(king);
    enemy_attacks.intersects(king) //&& (king_moves & !blockers & !enemy_attacks).is_empty()
}

fn is_check(position: &Position) -> bool {
    let king = position.get_piece(position.player_turn, Piece::King);
    let blockers = position.get_all_pieces();
    let sq = Square::try_from(king).unwrap(); //there should always be one king...
    let knight_moves = KNIGHT_MOVES[sq as usize];
    let rook_moves = rook_moves_pext(sq, blockers);
    let bishop_moves = bishop_moves_pext(sq, blockers);

    let pawn_moves = match position.player_turn {
        Colour::White => king.shift(Direction::LeftUp) | king.shift(Direction::RightUp),
        Colour::Black => king.shift(Direction::LeftDown) | king.shift(Direction::RightDown),
    };

    let check =
            pawn_moves.intersects(position.get_piece(position.player_turn.other(), Piece::Pawn))
        ||  knight_moves.intersects(position.get_piece(position.player_turn.other(), Piece::Knight))
        || rook_moves.intersects(position.get_piece(position.player_turn.other(), Piece::Rook))
        || bishop_moves.intersects(position.get_piece(position.player_turn.other(), Piece::Bishop))
        || (rook_moves | bishop_moves).intersects(position.get_piece(position.player_turn.other(), Piece::Queen));

    check
}