use crate::game::{Move};
use crate::move_gen::{generate_valid_moves, is_square_attacked_by_colour};
use crate::pieces::{Colour, Piece};
use crate::position::Position;
use crate::squares::Square;

pub static POSITION_SQUARE_EVALS: [[[i32; 64]; 6]; 2] = [
    [
        PAWN_MAP_WHITE,
        ROOK_MAP_WHITE,
        KNIGHT_MAP_WHITE,
        BISHOP_MAP_WHITE,
        QUEEN_MAP_WHITE,
        KING_MAP_WHITE
    ],
    [
        PAWN_MAP_BLACK,
        ROOK_MAP_BLACK,
        KNIGHT_MAP_BLACK,
        BISHOP_MAP_BLACK,
        QUEEN_MAP_BLACK,
        KING_MAP_BLACK
    ]
];

const PAWN_MAP_BLACK: [i32; 64] = flip(PAWN_MAP_WHITE);
const ROOK_MAP_BLACK: [i32; 64] = flip(ROOK_MAP_WHITE);
const KNIGHT_MAP_BLACK: [i32; 64] = flip(KNIGHT_MAP_WHITE);
const BISHOP_MAP_BLACK: [i32; 64] = flip(BISHOP_MAP_WHITE);
const QUEEN_MAP_BLACK: [i32; 64] = flip(QUEEN_MAP_WHITE);
const KING_MAP_BLACK: [i32; 64] = flip(KING_MAP_WHITE);

const MAX_DEPTH: u8 = 6;

macro_rules! heat_map {
      ($name:ident { $( [$($sq:expr),* ]),* $(,)?}) => {
          const $name: [i32; 64] = [$( $($sq,)* )*];
      };
}

const fn flip(map: [i32; 64]) -> [i32; 64] {
    let mut result = [0; 64];
    let mut i = 0;
    while i < 64 {
        result[i] = -1 * map[i ^ 0b111000];
        i += 1;
    }
    result
}

heat_map!( KNIGHT_MAP_WHITE {
    [-30, -30, -30, -30, -30, -30, -30, -30],
    [-30, 0, 0, 0, 0, 0, 0, -30],
    [-30, 0, 0, 0, 0, 0, 0, -30],
    [-30, 0, 0, 0, 0, 0, 0, -30],
    [-30, 20, 20, 20, 20, 20, 20, -30],
    [-30, 30, 30, 30, 30, 30, 30, -30],
    [-30, 30, 30, 30, 30, 30, 30, -30],
    [-30, -30, -30, -30, -30, -30, -30, -30],
});

heat_map!( PAWN_MAP_WHITE {
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 10, 10, 10, 10, 0, 0],
    [0, 0, 20, 20, 20, 20, 0, 0],
    [0, 0, 30, 30, 30, 30, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
});

heat_map!( BISHOP_MAP_WHITE {
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
});

heat_map!( ROOK_MAP_WHITE {
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [50, 50, 50, 50, 50, 50, 50, 50],
    [0, 0, 0, 0, 0, 0, 0, 0],
});

heat_map!( QUEEN_MAP_WHITE {
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 50, 50, 50, 50, 0, 0],
    [0, 0, 50, 50, 50, 50, 0, 0],
    [0, 0, 0, 50, 50, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
});

heat_map!( KING_MAP_WHITE {
    [0, 50, 50, 0, 0, 0, 50, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0],
});


pub fn get_best_move(position: &Position) -> (i32, Option<Move>) {
    let mut pos = position.clone();
    let node = search(MAX_DEPTH, &mut pos, i32::MIN, i32::MAX);
    (node.eval, node.mv)
}

fn search(depth: u8, position: &mut Position, mut alpha: i32, mut beta: i32) -> Node {
    if depth == 0 {
        return Node {
            mv: None,
            eval: position.eval,
        }
    }

    let mut best_score = match position.player_turn {
        Colour::White => i32::MIN,
        Colour::Black => i32::MAX
    };
    let mut best_move = None;

    let next_moves = generate_valid_moves(position);

    let mut count = 0;
    for mv in next_moves {
        if let Some(mv) = mv {
            let anti_move = position.make_reversible_move(mv);
            if is_square_attacked_by_colour(position, Square::try_from(position.get_piece(position.player_turn.other(), Piece::King)).unwrap(), position.player_turn) {
                position.reverse_move(anti_move);
                continue;
            }

            count += 1;
            let node = search(depth - 1, position, alpha, beta);
            position.reverse_move(anti_move);

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
        else {
            break;
        }
    }

    if count == 0 {
        return if is_square_attacked_by_colour(position, Square::try_from(position.get_piece(position.player_turn, Piece::King)).unwrap(), position.player_turn.other()) {
            match position.player_turn {
                Colour::White => {
                    Node {
                        mv: best_move,
                        eval: i32::MIN,
                    }
                },
                Colour::Black => {
                    Node {
                        mv: best_move,
                        eval: i32::MAX,
                    }
                }
            }
        } else {
            Node {
                mv: best_move,
                eval: 0,
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
    eval: i32,
}

mod test {
    use crate::engine::get_best_move;
    use crate::game::{Move, NormalMove};
    use crate::pieces::Piece;
    use crate::position::Position;
    use crate::squares::Square;
    use crate::move_gen::generate_valid_moves;


    fn assert_make_unmake_restores(pos: &mut Position, depth: u8, line: &mut Vec<Move>) {
        if depth == 0 {
            return;
        }
        for mv in generate_valid_moves(pos).into_iter().flatten() {
            let before = pos.clone();
            let anti_move = pos.make_reversible_move(mv);
            line.push(mv);
            assert_make_unmake_restores(pos, depth - 1, line);
            pos.reverse_move(anti_move);
            assert_eq!(*pos, before, "make/unmake mismatch after line {:?}", line);
            line.pop();
        }
    }

    #[test]
    fn make_unmake_restores_puzzle_tree() {
        let mut pos = Position::try_from_fen("5r1k/2p3pp/1rnb1qb1/p5N1/p2Pp3/2P4P/1PBQ1PP1/R1B1R1K1 b - - 0 1").unwrap();
        assert_make_unmake_restores(&mut pos, 3, &mut Vec::new());
    }

    #[test]
    fn make_unmake_restores_castling_tree() {
        let mut pos = Position::try_from_fen("r3k2r/pppq1ppp/2npbn2/2b1p3/2B1P3/2NPBN2/PPPQ1PPP/R3K2R w KQkq").unwrap();
        assert_make_unmake_restores(&mut pos, 3, &mut Vec::new());
    }

    #[test]
    fn puzzle_win_material_in_2() {
        let pos = Position::try_from_fen("5r1k/2p3pp/1rnb1qb1/p5N1/p2Pp3/2P4P/1PBQ1PP1/R1B1R1K1 b - - 0 1").unwrap();
        let result = get_best_move(&pos);
        let expected_mv = Move::Normal(
            NormalMove {
                from: Square::E4,
                to: Square::E3,
                piece: Piece::Pawn,
            }
        );
        assert_eq!(expected_mv, result.1.expect("No move returned"));
    }

    #[test]
    fn puzzle_mate_in_2() {
        let pos = Position::try_from_fen("2r1nr1k/1bp3qp/1p4p1/p4p2/PnBQ1N2/1P5P/1BP2RP1/6K1 w - - 0 1").unwrap();
        let result = get_best_move(&pos);
        let expected_mv = Move::Capture(
            NormalMove {
                from: Square::F4,
                to: Square::G6,
                piece: Piece::Knight,
            }
        );
        assert_eq!(expected_mv, result.1.expect("No move returned"));
    }

    #[test]
    fn puzzle_mate_in_2_quiet_move() {
        let pos = Position::try_from_fen("8/pR4p1/k2p2n1/2p1p1bp/1PP5/6PP/P4PBK/1r6 w - - 0 1").unwrap();
        let result = get_best_move(&pos);
        let expected_mv = Move::Normal(
            NormalMove {
                from: Square::B7,
                to: Square::B5,
                piece: Piece::Rook,
            }
        );
        assert_eq!(expected_mv, result.1.expect("No move returned"));
    }
}

