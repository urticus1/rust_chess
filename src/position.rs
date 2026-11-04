use std::fmt;
use std::fmt::Debug;
use crate::bitboard::{BitBoard, ROW_1, ROW_2, ROW_7, ROW_8};
use crate::engine::POSITION_SQUARE_EVALS;
use crate::game::{CastleSide, Move, NormalMove};
use crate::move_gen::is_square_attacked_by_colour;
use crate::squares::Square;
use crate::pieces::{Colour, Piece};

const STARTING_PAWNS_BLACK: BitBoard = ROW_7;
const STARTING_PAWNS_WHITE: BitBoard = ROW_2;

const STARTING_ROOKS_WHITE: BitBoard = BitBoard(0b_10000001);
const STARTING_KNIGHTS_WHITE: BitBoard = BitBoard(0b_01000010);

const STARTING_BISHOPS_WHITE: BitBoard = BitBoard(0b_00100100);
const STARTING_QUEEN_WHITE: BitBoard = BitBoard(0b_00001000);
const STARTING_KING_WHITE: BitBoard = BitBoard(0b_00010000);

const STARTING_ROOKS_BLACK: BitBoard = BitBoard(STARTING_ROOKS_WHITE.0 << 8 * 7 );
const STARTING_KNIGHTS_BLACK: BitBoard = BitBoard(STARTING_KNIGHTS_WHITE.0 << 8 * 7 );
const STARTING_BISHOPS_BLACK: BitBoard = BitBoard(STARTING_BISHOPS_WHITE.0 << 8 * 7 );
const STARTING_QUEEN_BLACK: BitBoard = BitBoard(STARTING_QUEEN_WHITE.0 << 8 * 7 );
const STARTING_KING_BLACK: BitBoard = BitBoard(STARTING_KING_WHITE.0 << 8 * 7 );

const WHITE_KINGSIDE_CASTLE_PATH: BitBoard = BitBoard(0b01110000u64);
const WHITE_QUEENSIDE_CASTLE_PATH: BitBoard = BitBoard(0b00011100u64);

const BLACK_KINGSIDE_CASTLE_PATH: BitBoard = BitBoard(0b01110000u64 << (8 * 7));
const BLACK_QUEENSIDE_CASTLE_PATH: BitBoard = BitBoard(0b00011100u64 << (8 * 7));

const CASTLE_PATHS: [[BitBoard; 2]; 2] = [
    [WHITE_KINGSIDE_CASTLE_PATH, WHITE_QUEENSIDE_CASTLE_PATH],
    [BLACK_KINGSIDE_CASTLE_PATH, BLACK_QUEENSIDE_CASTLE_PATH],
];

#[derive(Clone, PartialEq)]
pub struct Position {
    piece_boards: [[BitBoard; 6]; 2],
    all_pieces: [BitBoard; 2],
    pub player_turn: Colour,
    pub can_castle: [[bool; 2]; 2],
    pub eval: i32,
}

struct NormalAntiMove {
    from: Square,
    to: Square,
    piece: Piece,
    captured: Option<Piece>,
    kingside_castle: bool,
    queenside_castle: bool,
}

pub enum AntiMove {
    Normal (NormalAntiMove),
    Castle {
        side: CastleSide,
        kingside_castle: bool,
        queenside_castle: bool,
    }
}

#[derive(Debug, PartialEq)]
pub enum FenError {
    MissingField(&'static str),
    BadRankCount(usize),
    BadRankLength(usize),
    BadPiece(char),
    BadTurn(String),
    BadCastling(char),
}

impl Position {

    pub fn new_standard_game() -> Self {
        Position {
            piece_boards: [
                [
                    STARTING_PAWNS_WHITE,
                    STARTING_ROOKS_WHITE,
                    STARTING_KNIGHTS_WHITE,
                    STARTING_BISHOPS_WHITE,
                    STARTING_QUEEN_WHITE,
                    STARTING_KING_WHITE,
                ],
                [
                    STARTING_PAWNS_BLACK,
                    STARTING_ROOKS_BLACK,
                    STARTING_KNIGHTS_BLACK,
                    STARTING_BISHOPS_BLACK,
                    STARTING_QUEEN_BLACK,
                    STARTING_KING_BLACK,

                ]
            ],
            all_pieces: [ROW_1 | ROW_2, ROW_7 | ROW_8],
            player_turn: Colour::White,
            can_castle: [[true, true], [true, true]],
            eval: 0
        }
    }

    pub fn blank() -> Self {
        Position {
            piece_boards: [
                [BitBoard(0); 6],
                [BitBoard(0); 6]
            ],
            player_turn: Colour::White,
            all_pieces: [BitBoard(0); 2],
            can_castle: [[true, true], [true, true]],
            eval: 0
        }
    }

    pub fn try_from_fen(fen: &str) -> Result<Self, FenError> {
        let mut pos = Position::blank();
        let mut parts = fen.split_whitespace();

        let board = parts.next().ok_or(FenError::MissingField("board"))?;
        let ranks: Vec<&str> = board.split('/').collect();
        if ranks.len() != 8 {
            return Err(FenError::BadRankCount(ranks.len()));
        }

        // FEN lists rank 8 first
        for (i, rank_str) in ranks.iter().enumerate() {
            let rank = 7 - i;
            let mut file = 0;
            for c in rank_str.chars() {
                if let Some(empty) = c.to_digit(10) {
                    file += empty as usize;
                    continue;
                }
                if file >= 8 {
                    return Err(FenError::BadRankLength(rank + 1));
                }

                let colour = if c.is_ascii_uppercase() { Colour::White } else { Colour::Black };
                let piece = match c.to_ascii_uppercase() {
                    'P' => Piece::Pawn,
                    'R' => Piece::Rook,
                    'N' => Piece::Knight,
                    'B' => Piece::Bishop,
                    'Q' => Piece::Queen,
                    'K' => Piece::King,
                    _ => return Err(FenError::BadPiece(c)),
                };

                let sq = rank * 8 + file;
                let bit = BitBoard(1 << sq);
                let board = pos.get_piece_board(colour, piece);
                pos.set_bitboard(colour, piece, board | bit);
                pos.all_pieces[colour as usize] = pos.all_pieces[colour as usize] | bit;
                pos.eval += Piece::VALUES[colour as usize][piece as usize];
                pos.eval += POSITION_SQUARE_EVALS[colour as usize][piece as usize][sq];
                file += 1;
            }
            if file != 8 {
                return Err(FenError::BadRankLength(rank + 1));
            }
        }

        pos.player_turn = match parts.next() {
            Some("w") => Colour::White,
            Some("b") => Colour::Black,
            Some(other) => return Err(FenError::BadTurn(other.to_string())),
            None => return Err(FenError::MissingField("turn")),
        };

        pos.can_castle = [[false; 2]; 2];
        match parts.next() {
            Some("-") => {}
            Some(rights) => {
                for c in rights.chars() {
                    let (colour, side) = match c {
                        'K' => (Colour::White, CastleSide::King),
                        'Q' => (Colour::White, CastleSide::Queen),
                        'k' => (Colour::Black, CastleSide::King),
                        'q' => (Colour::Black, CastleSide::Queen),
                        _ => return Err(FenError::BadCastling(c)),
                    };
                    pos.can_castle[colour as usize][side as usize] = true;
                }
            }
            None => return Err(FenError::MissingField("castling")),
        }

        Ok(pos)
    }

    pub fn reverse_move(&mut self, mv: AntiMove) {
        match mv {
            AntiMove::Normal(mv) => self.reverse_normal_move(mv),
            AntiMove::Castle { side, kingside_castle, queenside_castle } => self.reverse_castle_move(side, kingside_castle, queenside_castle),
        }
    }

    fn reverse_castle_move(&mut self, side: CastleSide, kingside_castle: bool, queenside_castle: bool) {
        let colour = self.player_turn.other();
        self.player_turn = colour;
        self.can_castle[colour as usize][CastleSide::King as usize] = kingside_castle;
        self.can_castle[colour as usize][CastleSide::Queen as usize] = queenside_castle;

        let (rook_start, rook_end, king_start, king_end) = match (colour, side) {
            (Colour::White, CastleSide::King) => (Square::F1, Square::H1, Square::G1, Square::E1),
            (Colour::White, CastleSide::Queen) => (Square::D1, Square::A1, Square::C1, Square::E1),
            (Colour::Black, CastleSide::King) => (Square::F8, Square::H8, Square::G8, Square::E8),
            (Colour::Black, CastleSide::Queen) => (Square::D8, Square::A8, Square::C8, Square::E8),
        };

        self.piece_boards[colour as usize][Piece::King as usize] = king_end.to_bitboard();
        self.piece_boards[colour as usize][Piece::Rook as usize] = (self.piece_boards[colour as usize][Piece::Rook as usize] | rook_end.to_bitboard()) & !rook_start.to_bitboard();
        self.all_pieces[colour as usize] = (self.all_pieces[colour as usize] & !rook_start.to_bitboard() & !king_start.to_bitboard())
            | rook_end.to_bitboard() | king_end.to_bitboard();

        let king_evals = &POSITION_SQUARE_EVALS[colour as usize][Piece::King as usize];
        let rook_evals = &POSITION_SQUARE_EVALS[colour as usize][Piece::Rook as usize];
        self.eval -= king_evals[king_start as usize];
        self.eval += king_evals[king_end as usize];
        self.eval -= rook_evals[rook_start as usize];
        self.eval += rook_evals[rook_end as usize];
    }

    fn reverse_normal_move(&mut self, mv: NormalAntiMove) {
        if let Some(captured) = mv.captured {
            let board = self.get_piece_board(self.player_turn, captured);
            self.set_bitboard(self.player_turn, captured, board | mv.from.to_bitboard());
            self.all_pieces[self.player_turn as usize] = self.all_pieces[self.player_turn as usize] | mv.from.to_bitboard();
            self.eval += Piece::VALUES[self.player_turn as usize][captured as usize];
            self.eval += POSITION_SQUARE_EVALS[self.player_turn as usize][captured as usize][mv.from as usize];
        }
        self.player_turn = self.player_turn.other();
        self.can_castle[self.player_turn as usize][CastleSide::King as usize] = mv.kingside_castle;
        self.can_castle[self.player_turn as usize][CastleSide::Queen as usize] = mv.queenside_castle;

        let board = self.get_piece_board(self.player_turn, mv.piece);
        self.eval -= POSITION_SQUARE_EVALS[self.player_turn as usize][mv.piece as usize][mv.from as usize];
        self.eval += POSITION_SQUARE_EVALS[self.player_turn as usize][mv.piece as usize][mv.to as usize];
        self.set_bitboard(self.player_turn, mv.piece, (board | mv.to.to_bitboard()) & !mv.from.to_bitboard());
        self.all_pieces[self.player_turn as usize] = (self.all_pieces[self.player_turn as usize] | mv.to.to_bitboard()) & !mv.from.to_bitboard();
    }

    pub fn make_reversible_move(&mut self, mv: Move) -> AntiMove {
        match mv {
            Move::Normal(mv) => self.make_reversible_normal_move(mv),
            Move::Capture(mv) => self.make_reversible_normal_move(mv),
            Move::Check(mv) => self.make_reversible_normal_move(mv),
            Move::Castle(side, _) => self.make_castle_move(side)
        }
    }

    fn make_reversible_normal_move(&mut self, mv: NormalMove) -> AntiMove {
        let from = mv.from.to_bitboard();
        let to = mv.to.to_bitboard();


        let kingside_castle = self.can_castle[self.player_turn as usize][CastleSide::King as usize];
        let queenside_castle = self.can_castle[self.player_turn as usize][CastleSide::King as usize];

        let mut board = self.get_current_turn_boards()[mv.piece as usize];
        board = (board & !from) | to;
        self.eval -= POSITION_SQUARE_EVALS[self.player_turn as usize][mv.piece as usize][mv.from as usize];
        self.eval += POSITION_SQUARE_EVALS[self.player_turn as usize][mv.piece as usize][mv.to as usize];
        self.set_bitboard(self.player_turn, mv.piece, board);
        self.all_pieces[self.player_turn as usize] =  (self.all_pieces[self.player_turn as usize] & !from) | to;

        if mv.piece == Piece::King {
            self.can_castle[self.player_turn as usize][CastleSide::King as usize] = false;
            self.can_castle[self.player_turn as usize][CastleSide::Queen as usize] = false;
        }
        else if mv.piece == Piece::Rook {
            let rook_board = self.get_piece_board(self.player_turn, Piece::Rook);
            match self.player_turn {
                Colour::White => {
                    if (rook_board & Square::H1.to_bitboard()).0 != 0 {
                        self.can_castle[self.player_turn as usize][CastleSide::King as usize] = false;
                    }
                    else if (rook_board & Square::A1.to_bitboard()).0 != 0 {
                        self.can_castle[self.player_turn as usize][CastleSide::Queen as usize] = false;
                    }
                }
                Colour::Black => {
                    if (rook_board & Square::H8.to_bitboard()).0 != 0 {
                        self.can_castle[self.player_turn as usize][CastleSide::King as usize] = false;
                    }
                    else if (rook_board & Square::A8.to_bitboard()).0 != 0 {
                        self.can_castle[self.player_turn as usize][CastleSide::Queen as usize] = false;
                    }
                }
            }

        }

        let opponent = self.player_turn.other();
        self.player_turn = self.player_turn.other();
        let mut captured = None;
        for piece in Piece::ALL {
            if self.get_piece(opponent, *piece).intersects(to) {
                let b = self.get_piece_board(opponent, *piece);
                self.set_bitboard(opponent, *piece, b & !to);
                self.all_pieces[opponent as usize] =  self.all_pieces[opponent as usize] & !to;
                captured = Some(piece.clone());
                self.eval -= Piece::VALUES[opponent as usize][*piece as usize];
                self.eval -= POSITION_SQUARE_EVALS[opponent as usize][*piece as usize][mv.to as usize];
            }
        }
        AntiMove::Normal ( NormalAntiMove {
            from: mv.to,
            to: mv.from,
            piece: mv.piece,
            captured,
            kingside_castle,
            queenside_castle,
        })
    }

    pub fn get_piece(&self, colour: Colour, piece: Piece) -> BitBoard {
        self.piece_boards[colour as usize][piece as usize]
    }

    pub fn make_castle_move(&mut self, side: CastleSide) -> AntiMove {
        let colour = self.player_turn;
        let kingside_castle = self.can_castle[colour as usize][CastleSide::King as usize];
        let queenside_castle = self.can_castle[colour as usize][CastleSide::Queen as usize];
        self.can_castle[colour as usize][CastleSide::King as usize] = false;
        self.can_castle[colour as usize][CastleSide::Queen as usize] = false;

        let (rook_start, rook_end, king_start, king_end) = match (colour, side) {
            (Colour::White, CastleSide::King) => (Square::H1, Square::F1, Square::E1, Square::G1),
            (Colour::White, CastleSide::Queen) => (Square::A1, Square::D1, Square::E1, Square::C1),
            (Colour::Black, CastleSide::King) => (Square::H8, Square::F8, Square::E8, Square::G8),
            (Colour::Black, CastleSide::Queen) => (Square::A8, Square::D8, Square::E8, Square::C8),
        };

        let current_rooks = self.piece_boards[colour as usize][Piece::Rook as usize];
        self.piece_boards[colour as usize][Piece::Rook as usize] = (current_rooks & !rook_start.to_bitboard()) | rook_end.to_bitboard();
        self.piece_boards[colour as usize][Piece::King as usize] = king_end.to_bitboard();
        self.all_pieces[colour as usize] = (self.all_pieces[colour as usize] & !rook_start.to_bitboard() & !king_start.to_bitboard())
            | rook_end.to_bitboard() | king_end.to_bitboard();

        let king_evals = &POSITION_SQUARE_EVALS[colour as usize][Piece::King as usize];
        let rook_evals = &POSITION_SQUARE_EVALS[colour as usize][Piece::Rook as usize];
        self.eval -= king_evals[king_start as usize];
        self.eval += king_evals[king_end as usize];
        self.eval -= rook_evals[rook_start as usize];
        self.eval += rook_evals[rook_end as usize];

        self.player_turn = colour.other();

        AntiMove::Castle { side, kingside_castle, queenside_castle }
    }

    fn get_current_turn_boards(&self) -> &[BitBoard; 6] {
        &self.piece_boards[self.player_turn as usize]
    }

    pub fn get_sides_pieces(&self, colour: Colour) -> BitBoard {
        self.all_pieces[colour as usize]
    }

    pub fn get_all_pieces(&self) -> BitBoard {
        self.all_pieces[0] | self.all_pieces[1]
    }

    pub fn set_bitboard(&mut self, colour: Colour, piece: Piece, value: BitBoard) {
        self.piece_boards[colour as usize][piece as usize] = value;
    }

    pub fn get_piece_board(&self, colour: Colour, piece: Piece) -> BitBoard {
        self.piece_boards[colour as usize][piece as usize]
    }

    pub fn can_castle(&self, colour: Colour, side: CastleSide) -> bool {
        let path = CASTLE_PATHS[colour as usize][side as usize];
        if !self.can_castle[colour as usize][side as usize] {
            return false;
        }

        for sq in path {
            if is_square_attacked_by_colour(self, sq, colour.other()) {
                return false;
            }
        }
        let king = match colour {
            Colour::White => Square::E1.to_bitboard(),
            Colour::Black => Square::E8.to_bitboard(),
        };
        if (path & !king).intersects(self.get_all_pieces()) {
            return false;
        }
        true
    }

    pub fn get_pseudo_moves_single_piece_validated(&self, colour: Colour, piece: Piece, square: Square) -> Option<BitBoard> {
        let board = self.piece_boards[colour as usize][piece as usize];

        let pos = square.to_bitboard() & board;
        if pos.is_empty() {
            return None;
        }
        let pseudo_moves = piece.get_pseudo_moves(&self, pos.try_into().unwrap(), self.get_all_pieces(), colour);
        Some(pseudo_moves)
    }

    pub fn get_pseudo_moves_single_piece(&self, colour: Colour, piece: Piece, square: Square) -> BitBoard {
        piece.get_pseudo_moves(&self, square, self.get_all_pieces(), colour)
    }

    fn piece_char_at(&self, square: BitBoard) -> char {
        for (colour, boards) in self.piece_boards.iter().enumerate() {
            for (piece, board) in boards.iter().enumerate() {
                if board.intersects(square) {
                    let letter = PIECE_LETTERS[piece];
                    return if colour == Colour::Black as usize { letter.to_ascii_lowercase() } else { letter };
                }
            }
        }
        '.'
    }
}

const PIECE_LETTERS: [char; 6] = ['P', 'R', 'N', 'B', 'Q', 'K'];

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n\n\n")?;
        for rank in (0..8).rev() {
            write!(f, "{} ", rank + 1)?;
            for file in 0..8 {
                let square = BitBoard(1u64 << (rank * 8 + file));
                write!(f, "{} ", self.piece_char_at(square))?;
            }
            writeln!(f)?;
        }
        write!(f, "  ")?;
        for file in b'A'..=b'H' {
            write!(f, "{} ", file as char)?;
        }
        write!(f, "\n\n\n")
    }
}

impl Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n\n\n")?;
        for rank in (0..8).rev() {
            write!(f, "{} ", rank + 1)?;
            for file in 0..8 {
                let square = BitBoard(1u64 << (rank * 8 + file));
                write!(f, "{} ", self.piece_char_at(square))?;
            }
            writeln!(f)?;
        }
        write!(f, "  ")?;
        for file in b'A'..=b'H' {
            write!(f, "{} ", file as char)?;
        }
        write!(f, "\n\n\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pieces::Piece::{Bishop, Knight, Pawn, Queen};

    /// Plays normal moves from the starting position, panicking if any of them fail.
    fn play(moves: &[(Piece, Square, Square)]) -> Position {
        let mut pos = Position::new_standard_game();
        for &(piece, from, to) in moves {
            pos.make_reversible_move(Move::Normal(NormalMove { piece, from, to }));
        }
        pos
    }

    fn castle(pos: &mut Position, side: CastleSide, colour: Colour) {
        pos.make_reversible_move(Move::Castle(side, colour));
    }

    fn assert_squares(pos: &Position, colour: Colour, piece: Piece, squares: &[Square]) {
        let expected = squares.iter().fold(0u64, |acc, sq| acc | sq.to_bitboard().0);
        let actual = pos.get_piece(colour, piece);
        assert_eq!(actual,  BitBoard(expected), "{:?} board: got {}, expected {}", piece, actual, BitBoard(expected));
    }

    const KINGSIDE_OPENING: [(Piece, Square, Square); 6] = [
        (Pawn, Square::E2, Square::E4),
        (Pawn, Square::E7, Square::E5),
        (Knight, Square::G1, Square::F3),
        (Knight, Square::G8, Square::F6),
        (Bishop, Square::F1, Square::C4),
        (Bishop, Square::F8, Square::C5),
    ];

    const QUEENSIDE_OPENING: [(Piece, Square, Square); 8] = [
        (Pawn, Square::D2, Square::D4),
        (Pawn, Square::D7, Square::D5),
        (Knight, Square::B1, Square::C3),
        (Knight, Square::B8, Square::C6),
        (Bishop, Square::C1, Square::F4),
        (Bishop, Square::C8, Square::F5),
        (Queen, Square::D1, Square::D2),
        (Queen, Square::D8, Square::D7),
    ];

    #[test]
    fn white_castles_kingside() {
        let mut pos = play(&KINGSIDE_OPENING);
        castle(&mut pos, CastleSide::King, Colour::White);

        assert_squares(&pos, Colour::White, Piece::King, &[Square::G1]);
        assert_squares(&pos, Colour::White, Piece::Rook, &[Square::A1, Square::F1]);
        assert!(!pos.can_castle[Colour::White as usize][CastleSide::King as usize]);
        assert!(!pos.can_castle[Colour::White as usize][CastleSide::Queen as usize]);
    }

    #[test]
    fn black_castles_kingside() {
        let mut moves = KINGSIDE_OPENING.to_vec();
        moves.push((Pawn, Square::D2, Square::D3));
        let mut pos = play(&moves);
        castle(&mut pos, CastleSide::King, Colour::Black);

        assert_squares(&pos, Colour::Black, Piece::King, &[Square::G8]);
        assert_squares(&pos, Colour::Black, Piece::Rook, &[Square::A8, Square::F8]);
    }

    #[test]
    fn white_castles_queenside() {
        let mut pos = play(&QUEENSIDE_OPENING);
        castle(&mut pos, CastleSide::Queen, Colour::White);

        assert_squares(&pos, Colour::White, Piece::King, &[Square::C1]);
        assert_squares(&pos, Colour::White, Piece::Rook, &[Square::D1, Square::H1]);
    }

    #[test]
    fn black_castles_queenside() {
        let mut moves = QUEENSIDE_OPENING.to_vec();
        moves.push((Pawn, Square::E2, Square::E3));
        let mut pos = play(&moves);
        castle(&mut pos, CastleSide::Queen, Colour::Black);

        assert_squares(&pos, Colour::Black, Piece::King, &[Square::C8]);
        assert_squares(&pos, Colour::Black, Piece::Rook, &[Square::D8, Square::H8]);
    }

    #[test]
    fn capture_then_reverse_restores_position() {
        let mut pos = play(&[
            (Pawn, Square::E2, Square::E4),
            (Pawn, Square::D7, Square::D5),
        ]);
        let before = pos.clone();

        let anti_move = pos.make_reversible_move(Move::Capture(NormalMove {
            piece: Pawn,
            from: Square::E4,
            to: Square::D5,
        }));

        assert_squares(&pos, Colour::White, Pawn, &[
            Square::A2, Square::B2, Square::C2, Square::D2, Square::D5, Square::F2, Square::G2, Square::H2,
        ]);
        assert!(!pos.get_piece(Colour::Black, Pawn).intersects(Square::D5.to_bitboard()));
        assert_ne!(pos.eval, before.eval);

        pos.reverse_move(anti_move);

        assert_eq!(pos, before);
    }

    #[test]
    fn castling_blocked_in_starting_position() {
        let pos = Position::new_standard_game();

        assert_eq!(pos.can_castle(Colour::White, CastleSide::King), false);
        assert_eq!(pos.can_castle(Colour::White, CastleSide::Queen), false);
    }


    #[test]
    fn fen_standard_position() {
        let pos = Position::try_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq").unwrap();
        assert_eq!(pos, Position::new_standard_game());
    }

    #[test]
    fn fen_ok() {
        let pos = Position::try_from_fen("5r1k/2p3pp/1rnb1qb1/p5N1/p2Pp3/2P4P/1PBQ1PP1/R1B1R1K1 b - - 0 1");
        println!("{:?}", pos);
        assert_eq!(true, pos.is_ok());
    }
}
