use crate::game::Move;
use crate::move_gen::generate_valid_moves;
use crate::pieces::{king_moves, Colour, Piece};
use crate::position::Position;

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
    [0.0, 0.0, 0.2, 0.2, 0.2, 0.2, 0.0, 0.0],
    [0.0, 0.0, 0.3, 0.3, 0.3, 0.3, 0.0, 0.0],
    [0.0, 0.0, 0.4, 0.4, 0.4, 0.4, 0.0, 0.0],
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


pub fn eval(position: &Position) -> f32 {

    let positional_score: f32 = Piece::ALL.iter()
        .flat_map(|piece| position.get_piece(position.player_turn, *piece).map(move |sq| (sq, piece)))
        .map(|(sq, piece)| {
            let heat_map = match piece {
                Piece::Pawn => PAWN_MAP,
                Piece::Knight => KNIGHT_MAP,
                _ => BLANK_MAP
            };
            let index = if position.player_turn == Colour::White { sq as usize } else { 63 - sq as usize };
            heat_map[index]
        })
        .map(|score| {
            if position.player_turn == Colour::Black {
                return score * -1.0;
            }
            score
        })
        .sum();

    material_score(position, Colour::White) - material_score(position, Colour::Black) + positional_score
}

pub fn get_best_move(position: &Position) -> (f32, Option<Move>) {
    let start_pos = position.clone();
    search(0, &start_pos)
}

fn search(depth: u8, position: &Position) -> (f32, Option<Move>) {
    if depth >= 4 {
        return (eval(position), None);
    }

    let mut best_score = match position.player_turn {
        Colour::White => -f32::INFINITY,
        Colour::Black => f32::INFINITY
    };
    let mut best_move = None;

    for mv in generate_valid_moves(position) {
        let new_pos = position.make_move(mv);
        if let Ok(valid_pos) = new_pos {
            let (eval, _) = search(depth + 1, &valid_pos);
            match position.player_turn {
                Colour::White => {
                    if eval >= best_score {
                        best_score = eval;
                        best_move = Some(mv);
                    }
                },
                Colour::Black => {
                    if eval <= best_score {
                        best_score = eval;
                        best_move = Some(mv);
                    }
                }
            }
        }

    }

    (best_score, best_move)
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