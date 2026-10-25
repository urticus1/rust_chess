use std::arch::x86_64::_pext_u64;
use std::sync::LazyLock;
use crate::bitboard::{BitBoard};
use crate::{INTERIOR_SQUARES, ROW_2, ROW_7};
use crate::pieces::Piece::{Bishop, King, Knight, Pawn, Queen, Rook};
use crate::squares::{Direction, Square};
use crate::position::Position;
use crate::squares::Square::H8;

const fn create_knight_moves() -> [BitBoard; 64] {
    let mut boards = [BitBoard(0); 64];
    let mut i = 0;
    while i < 64 {
        boards[i] = knight_moves(BitBoard(1 << i));
        i+= 1;
    }
    boards
}

const fn create_king_moves() -> [BitBoard; 64] {
    let mut boards = [BitBoard(0); 64];
    let mut i = 0;
    while i < 64 {
        boards[i] = king_moves(BitBoard(1 << i));
        i+= 1;
    }
    boards
}

pub fn generate_all_rook_move_sets() -> Vec<BitBoard> {
    let mut moves = vec![];
    for sq in BitBoard(u64::MAX) {
        for blockers in generate_rook_blockers_from_square(sq) {
            moves.push(rook_moves(sq, blockers));
        }
    }
    moves
}

pub fn generate_all_bishop_move_sets() -> Vec<BitBoard> {
    let size = 2usize.pow(9);
    let mut moves = vec![BitBoard(0); size * 64];
    for sq in BitBoard(u64::MAX) {
        let offset = (sq as usize) * size;
        for (i, blockers) in generate_bishop_blockers_from_square(sq).into_iter().enumerate() {
            moves[i + offset] = bishop_moves(sq, blockers);
        }
    }
    moves
}

const ROOK_OFFSETS: [usize; 64] = [
    0, 4096, 8192, 12288, 16384, 20480, 24576, 28672,
    32768, 36864, 37888, 38912, 39936, 40960, 41984, 43008,
    47104, 51200, 52224, 53248, 54272, 55296, 56320, 57344,
    61440, 65536, 66560, 67584, 68608, 69632, 70656, 71680,
    75776, 79872, 80896, 81920, 82944, 83968, 84992, 86016,
    90112, 94208, 95232, 96256, 97280, 98304, 99328, 100352,
    104448, 108544, 109568, 110592, 111616, 112640, 113664, 114688,
    118784, 122880, 126976, 131072, 135168, 139264, 143360, 147456,
];

pub fn rook_moves_pext(sq: Square, blockers: BitBoard) -> BitBoard {
    let (row, col) = sq.row_col();
    let mask = (row | col) & !(sq.board_edge_squares() ^ sq.to_bitboard()) & !sq.to_bitboard(); //TODO cache these
    unsafe {
        let i = _pext_u64(blockers.0, mask.0) as usize + ROOK_OFFSETS[sq as usize];
        return ROOK_MOVES[i];
    }
}

pub fn bishop_moves_pext(sq: Square, blockers: BitBoard) -> BitBoard {
    let mask = DIAGONALS[sq as usize] & INTERIOR_SQUARES & !sq.to_bitboard();
    let offset = (sq as usize) * 2_usize.pow(9);
    unsafe {
        let i = _pext_u64(blockers.0, mask.0) as usize + offset;
        BISHOP_MOVES[i]
    }
}

fn generate_rook_blockers_from_square(square: Square) -> Vec<BitBoard> {
    let size = if square == H8 { 4096 } else { ROOK_OFFSETS[square as usize + 1] - ROOK_OFFSETS[square as usize] };
    let mut blockers = vec![BitBoard(0); size];
    let mut permutation = 0u64;
    let (row, col) = square.row_col();
    //println!("{:?} {}",square, square.board_edge_squares());
    let mask = (row | col) & !(square.board_edge_squares() ^ square.to_bitboard()) & !square.to_bitboard();
    //println!("{}", mask);
    let mask = mask.0;
    loop {

        let b = permutation.wrapping_sub(mask) & mask;
        permutation = b;

        unsafe {
            let i = _pext_u64(permutation, mask) as usize;
            blockers[i] = BitBoard(b);
        }

        if permutation == 0 {
            break;
        }
    }
    blockers
}

fn generate_bishop_blockers_from_square(square: Square) -> Vec<BitBoard> {
    let size = 2_usize.pow(9);
    let mut blockers = vec![BitBoard(0); size];
    let mut permutation = 0u64;
    let diagonals = DIAGONALS[square as usize];
    //println!("{:?} {}",square, square.board_edge_squares());
    let mask = diagonals & INTERIOR_SQUARES & !square.to_bitboard();
    //println!("{}", mask);
    let mask = mask.0;
    loop {
        let b = permutation.wrapping_sub(mask) & mask;
        permutation = b;

        unsafe {
            let i = _pext_u64(permutation, mask) as usize;
            blockers[i] = BitBoard(b);
        }

        if permutation == 0 {
            break;
        }
    }
    blockers
}

fn generate_diagonal_masks() -> [BitBoard; 64] {
    let mut masks = [BitBoard(0u64); 64];
    BitBoard(u64::MAX).into_iter().enumerate().for_each(|(i, sq)| {
        masks[i] = sq.diagonals();
    });
    masks
}

pub const KNIGHT_MOVES: [BitBoard; 64] = create_knight_moves();
const KING_MOVES: [BitBoard; 64] = create_king_moves();
pub static ROOK_MOVES: LazyLock<Vec<BitBoard>> = LazyLock::new(generate_all_rook_move_sets);
pub static BISHOP_MOVES: LazyLock<Vec<BitBoard>> = LazyLock::new(generate_all_bishop_move_sets);

static DIAGONALS: LazyLock<[BitBoard; 64]> = LazyLock::new(generate_diagonal_masks);

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Piece {
    Pawn = 0,
    Rook = 1,
    Knight = 2,
    Bishop = 3,
    Queen = 4,
    King = 5
}

impl Piece {
    pub const ALL: &'static [Piece; 6] = &[Pawn, Rook, Knight, Bishop, Queen, King];
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Colour {
    White = 0,
    Black = 1,
}

impl Colour {
    pub fn other(&self) -> Colour {
        match self {
            Colour::White => Colour::Black,
            Colour::Black => Colour::White,
        }
    }
}

impl Piece {
    pub fn get_pseudo_moves(&self, position: &Position, piece_pos: Square, pieces: BitBoard, colour: Colour) -> BitBoard {

        match self {
            Piece::Pawn => single_pawn_moves(piece_pos.to_bitboard(), position, colour) | (single_pawn_pseudo_attacks(piece_pos.to_bitboard(), colour) & position.get_sides_pieces(colour.other())),
            Piece::Knight => KNIGHT_MOVES[piece_pos as usize],
            Piece::Bishop => bishop_moves_pext(piece_pos, pieces),
            Piece::Rook => rook_moves_pext(piece_pos, pieces),
            Piece::Queen => bishop_moves_pext(piece_pos, pieces) | rook_moves_pext(piece_pos, pieces),
            Piece::King => KING_MOVES[piece_pos as usize],
        }
    }
}

fn single_pawn_moves(pawn: BitBoard, pos: &Position, colour: Colour) -> BitBoard {
    let blockers = pos.get_all_pieces();
    match colour {
        Colour::White => {
            let first_move = (pawn & ROW_2).shift(Direction::Up) & !blockers;
            let remaining_moves = (first_move | pawn).shift(Direction::Up) & !blockers;
            first_move | remaining_moves
        },
        Colour::Black => {
            let first_move = (pawn & ROW_7).shift(Direction::Down) & !blockers;
            let remaining_moves = (first_move | pawn).shift(Direction::Down) & !blockers;
            first_move | remaining_moves
        }
    }
}

fn single_pawn_pseudo_attacks(pawn: BitBoard, colour: Colour) -> BitBoard {
    match colour {
        Colour::White => pawn.shift(Direction::LeftUp) | pawn.shift(Direction::RightUp),
        Colour::Black => pawn.shift(Direction::LeftDown) | pawn.shift(Direction::RightDown),
    }
}
pub fn all_pawn_attacks(pos: &Position, colour: Colour) -> BitBoard {
    let board = pos.get_piece_board(colour, Piece::Pawn);

    match colour {
        Colour::White => board.shift(Direction::LeftUp) | board.shift(Direction::RightUp),
        Colour::Black => board.shift(Direction::LeftDown) | board.shift(Direction::RightDown),
    }
}

//TODO add castling in here?
const fn king_moves(pos: BitBoard) -> BitBoard {
    let board = pos.shift(Direction::Up).0
    | pos.shift(Direction::Down).0
    | pos.shift(Direction::Left).0
    | pos.shift(Direction::Right).0
    | pos.shift(Direction::LeftUp).0
    | pos.shift(Direction::RightDown).0
    | pos.shift(Direction::LeftDown).0
    | pos.shift(Direction::RightUp).0;
    BitBoard(board)
}

const fn knight_moves(pos: BitBoard) -> BitBoard {
    let board = pos.shift(Direction::Up).shift(Direction::Up).shift(Direction::Right).0
        | pos.shift(Direction::Up).shift(Direction::Up).shift(Direction::Left).0
        | pos.shift(Direction::Down).shift(Direction::Down).shift(Direction::Right).0
        | pos.shift(Direction::Down).shift(Direction::Down).shift(Direction::Left).0

        | pos.shift(Direction::Left).shift(Direction::Left).shift(Direction::Up).0
        | pos.shift(Direction::Left).shift(Direction::Left).shift(Direction::Down).0
        | pos.shift(Direction::Right).shift(Direction::Right).shift(Direction::Up).0
        | pos.shift(Direction::Right).shift(Direction::Right).shift(Direction::Down).0;
    BitBoard(board)
}

pub fn rook_moves(pos: Square, pieces: BitBoard) -> BitBoard {
    let mut result = BitBoard (0 );
    for dir in [Direction::Up, Direction::Down, Direction::Right, Direction::Left] {
        let mut next = pos.get_neighbour(&dir);
        while next.is_some() {
            let current = next.unwrap();
            let pos_bitboard = current.to_bitboard();
            result = &result | &pos_bitboard;
            next = current.get_neighbour(&dir);
            if (pieces & pos_bitboard).0 != 0 {
                break;
            }
        }
    }
    result
}

pub fn queen_moves(pos: Square, pieces: BitBoard) -> BitBoard {
    let mut result = BitBoard (0 );
    for dir in [Direction::LeftUp, Direction::LeftDown, Direction::RightUp, Direction::RightDown, Direction::Down, Direction::Up, Direction::Left, Direction::Right] {
        let mut next = pos.get_neighbour(&dir);
        while next.is_some() {
            let current = next.unwrap();
            let pos_bitboard = current.to_bitboard();
            result = &result | &pos_bitboard;
            next = current.get_neighbour(&dir);
            if (pieces & pos_bitboard).0 != 0 {
                break;
            }
        }
    }
    result
}

pub fn bishop_moves(pos: Square, pieces: BitBoard) -> BitBoard {
    let mut result = BitBoard (0);
    for dir in [Direction::LeftUp, Direction::LeftDown, Direction::RightUp, Direction::RightDown] {
        let mut next = pos.get_neighbour(&dir);
        while next.is_some() {
            let current = next.unwrap();
            let pos_bitboard = current.to_bitboard();
            result = &result | &pos_bitboard;
            next = current.get_neighbour(&dir);
            if (pieces & pos_bitboard).0 != 0 {
                break;
            }
        }
    }
    result
}


#[cfg(test)]
mod tests {
    use crate::pieces::{Colour, Piece};
    use crate::position::Position;
    use crate::squares::Square;

    #[test]
    fn test_rook_captures() {

        let mut pos = Position::blank();
        pos.update_bitboard(Colour::White, Piece::Rook, Square::A1.to_bitboard());

        pos.update_bitboard(Colour::White, Piece::Queen, Square::A2.to_bitboard());
        pos.update_bitboard(Colour::Black, Piece::Queen, Square::B1.to_bitboard());

        let moves = pos.get_moves(Colour::White, Piece::Rook);

        assert!(moves.is_some());
        assert!(Square::B1.to_bitboard() == moves.unwrap())
    }

    mod rook_pext {
        use crate::bitboard::BitBoard;
        use crate::pieces::{generate_all_rook_move_sets, generate_rook_blockers_from_square, rook_moves, rook_moves_pext};
        use crate::squares::Square;
        use crate::squares::Square::*;

        fn bmi2_available() -> bool {
            is_x86_feature_detected!("bmi2")
        }

        fn board(squares: &[Square]) -> BitBoard {
            squares.iter().fold(BitBoard(0), |acc, sq| acc | sq.to_bitboard())
        }

        fn pext(sq: Square, blockers: BitBoard, table: &Vec<BitBoard>) -> BitBoard {
            unsafe { rook_moves_pext(sq, blockers, table) }
        }

        #[test]
        fn empty_board_from_corner() {
            if !bmi2_available() { return; }
            let table = generate_all_rook_move_sets();

            let expected = board(&[B1, C1, D1, E1, F1, G1, H1, A2, A3, A4, A5, A6, A7, A8]);
            let actual = pext(A1, BitBoard(0), &table);
            assert_eq!(actual, expected, "expected = {} actual = {}", expected, actual);
        }

        #[test]
        fn empty_board_from_centre() {
            if !bmi2_available() { return; }
            let table = generate_all_rook_move_sets();

            let expected = board(&[A4, B4, C4, E4, F4, G4, H4, D1, D2, D3, D5, D6, D7, D8]);
            let actual = pext(D4, BitBoard(0), &table);
            assert_eq!(actual, expected, "expected = {} actual = {}", expected, actual);
        }

        #[test]
        fn blocked_in_every_direction() {
            if !bmi2_available() { return; }
            let table = generate_all_rook_move_sets();

            let blockers = board(&[D6, B4, D2, G4]);
            // blocker squares are included: they may be captures
            let expected = board(&[D5, D6, C4, B4, D3, D2, E4, F4, G4]);
            let actual = pext(D4, blockers, &table);
            assert_eq!(actual, expected, "expected = {} actual = {}", expected, actual);
        }

        #[test]
        fn adjacent_blockers_box_rook_in() {
            if !bmi2_available() { return; }
            let table = generate_all_rook_move_sets();

            let blockers = board(&[D5, D3, C4, E4]);
            let expected = blockers;
            let actual = pext(D4, blockers, &table);
            assert_eq!(actual, expected, "expected = {} actual = {}", expected, actual);
        }

        #[test]
        fn blockers_on_edge_squares_are_ignored() {
            if !bmi2_available() { return; }
            let table = generate_all_rook_move_sets();

            // the rook can reach the edge either way, so these must not change the result
            let edge_blockers = board(&[D1, D8, A4, H4]);
            let expected = pext(D4, BitBoard(0), &table);
            let actual = pext(D4, edge_blockers, &table);
            assert_eq!(actual, expected, "expected = {} actual = {}", expected, actual);
        }

        #[test]
        fn blockers_off_the_rooks_lines_are_ignored() {
            if !bmi2_available() { return; }
            let table = generate_all_rook_move_sets();

            let unrelated = board(&[A1, B2, C3, E5, F6, G7, H8, E2, F7]);
            let expected = pext(D4, BitBoard(0), &table);
            let actual = pext(D4, unrelated, &table);
            assert_eq!(actual, expected, "expected = {} actual = {}", expected, actual);
        }

        #[test]
        fn rook_on_edge_with_blocker() {
            if !bmi2_available() { return; }
            let table = generate_all_rook_move_sets();

            let blockers = board(&[A4, C1]);
            let expected = board(&[A4, A3, A2, B1, C1]);
            let actual = pext(A1, blockers, &table);
            assert_eq!(actual, expected, "expected = {} actual = {}", expected, actual);
        }

        #[test]
        fn matches_rook_moves_for_every_square_and_blocker_set() {
            if !bmi2_available() { return; }
            let table = generate_all_rook_move_sets();

            for sq in BitBoard(u64::MAX) {
                for blockers in generate_rook_blockers_from_square(sq) {
                    let expected = rook_moves(sq, blockers);
                    let actual = pext(sq, blockers, &table);
                    assert_eq!(
                        actual, expected,
                        "square {:?}, blockers {} expected = {} actual = {}", sq, blockers, expected, actual
                    );
                }
            }
        }

    }

    mod bishop_pext {
        use crate::bitboard::BitBoard;
        use crate::pieces::{bishop_moves, bishop_moves_pext, generate_bishop_blockers_from_square};
        use crate::squares::Square;
        use crate::squares::Square::*;

        fn bmi2_available() -> bool {
            is_x86_feature_detected!("bmi2")
        }

        fn board(squares: &[Square]) -> BitBoard {
            squares.iter().fold(BitBoard(0), |acc, sq| acc | sq.to_bitboard())
        }

        #[test]
        fn empty_board_from_corner() {
            if !bmi2_available() { return; }

            let expected = board(&[B2, C3, D4, E5, F6, G7, H8]);
            let actual = bishop_moves_pext(A1, BitBoard(0));
            assert_eq!(actual, expected, "expected = {} actual = {}", expected, actual);
        }

        #[test]
        fn empty_board_from_centre() {
            if !bmi2_available() { return; }

            let expected = board(&[A1, B2, C3, E5, F6, G7, H8, A7, B6, C5, E3, F2, G1]);
            let actual = bishop_moves_pext(D4, BitBoard(0));
            assert_eq!(actual, expected, "expected = {} actual = {}", expected, actual);
        }

        #[test]
        fn blocked_in_every_direction() {
            if !bmi2_available() { return; }

            let blockers = board(&[F6, B6, B2, F2]);
            // blocker squares are included: they may be captures
            let expected = board(&[E5, F6, C5, B6, C3, B2, E3, F2]);
            let actual = bishop_moves_pext(D4, blockers);
            assert_eq!(actual, expected, "expected = {} actual = {}", expected, actual);
        }

        #[test]
        fn adjacent_blockers_box_bishop_in() {
            if !bmi2_available() { return; }

            let blockers = board(&[C5, E5, C3, E3]);
            let expected = blockers;
            let actual = bishop_moves_pext(D4, blockers);
            assert_eq!(actual, expected, "expected = {} actual = {}", expected, actual);
        }

        #[test]
        fn interior_blocker_from_corner() {
            if !bmi2_available() { return; }

            let blockers = board(&[D4]);
            let expected = board(&[B2, C3, D4]);
            let actual = bishop_moves_pext(A1, blockers);
            assert_eq!(actual, expected, "expected = {} actual = {}", expected, actual);
        }

        #[test]
        fn blockers_on_edge_squares_are_ignored() {
            if !bmi2_available() { return; }

            // the bishop reaches the end of each diagonal either way, so these must not change the result
            let edge_blockers = board(&[A1, H8, A7, G1]);
            let expected = bishop_moves_pext(D4, BitBoard(0));
            let actual = bishop_moves_pext(D4, edge_blockers);
            assert_eq!(actual, expected, "expected = {} actual = {}", expected, actual);
        }

        #[test]
        fn blockers_off_the_diagonals_are_ignored() {
            if !bmi2_available() { return; }

            let unrelated = board(&[D5, D3, C4, E4, A4, H4, D1, D8, B3, F5]);
            let expected = bishop_moves_pext(D4, BitBoard(0));
            let actual = bishop_moves_pext(D4, unrelated);
            assert_eq!(actual, expected, "expected = {} actual = {}", expected, actual);
        }

        #[test]
        fn bishop_on_edge_with_blockers() {
            if !bmi2_available() { return; }

            let blockers = board(&[F6, F2]);
            let expected = board(&[G5, F6, G3, F2]);
            let actual = bishop_moves_pext(H4, blockers);
            assert_eq!(actual, expected, "expected = {} actual = {}", expected, actual);
        }

        #[test]
        fn matches_bishop_moves_for_every_square_and_blocker_set() {
            if !bmi2_available() { return; }

            for sq in BitBoard(u64::MAX) {
                for blockers in generate_bishop_blockers_from_square(sq) {
                    let expected = bishop_moves(sq, blockers);
                    let actual = bishop_moves_pext(sq, blockers);
                    assert_eq!(
                        actual, expected,
                        "square {:?}, blockers {} expected = {} actual = {}", sq, blockers, expected, actual
                    );
                }
            }
        }

        #[test]
        fn matches_bishop_moves_for_random_full_boards() {
            if !bmi2_available() { return; }

            // the exhaustive test above shares its mask with the lookup, so it can't catch a bad mask;
            // random full-board occupancies are independent of the mask
            let mut state = 0x9E37_79B9_7F4A_7C15u64;
            let mut next_random = || {
                // xorshift64
                state ^= state << 13;
                state ^= state >> 7;
                state ^= state << 17;
                state
            };

            for _ in 0..1000 {
                // AND two randoms together so roughly a quarter of squares are occupied
                let blockers = BitBoard(next_random() & next_random());
                for sq in BitBoard(u64::MAX) {
                    let expected = bishop_moves(sq, blockers);
                    let actual = bishop_moves_pext(sq, blockers);
                    assert_eq!(
                        actual, expected,
                        "square {:?}, blockers {} expected = {} actual = {}", sq, blockers, expected, actual
                    );
                }
            }
        }
    }
}