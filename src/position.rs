use std::fmt;
use crate::bitboard::{BitBoard, ROW_2, ROW_7};
use crate::game::{CastleSide, Move, MoveError, NormalMove};
use crate::squares::Square;
use crate::pieces::{all_pawn_attacks, Colour, Piece};
use crate::squares::Square::{A1, A2};

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

const WHITE_KINGSIDE_CASTLE_PATH: BitBoard = BitBoard(0b01100000u64);
const WHITE_QUEENSIDE_CASTLE_PATH: BitBoard = BitBoard(0b00001100u64);

const BLACK_KINGSIDE_CASTLE_PATH: BitBoard = BitBoard(0b01100000u64 << (8 * 7));
const BLACK_QUEENSIDE_CASTLE_PATH: BitBoard = BitBoard(0b00001100u64 << (8 * 7));

const CASTLE_PATHS: [[BitBoard; 2]; 2] = [
    [WHITE_KINGSIDE_CASTLE_PATH, WHITE_QUEENSIDE_CASTLE_PATH],
    [BLACK_KINGSIDE_CASTLE_PATH, BLACK_QUEENSIDE_CASTLE_PATH],
];

#[derive(Clone)]
pub struct Position {
    piece_boards: [[BitBoard; 6]; 2],
    pub player_turn: Colour,
    pub can_castle: [[bool; 2]; 2],
}

pub struct AntiMove {
    from: Square,
    to: Square,
    piece: Piece,
    captured: Option<Piece>,
    kingside_castle: bool,
    queenside_castle: bool,
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
            player_turn: Colour::White,
            can_castle: [[true, true], [true, true]],
        }
    }

    pub fn blank() -> Self {
        Position {
            piece_boards: [
                [BitBoard(0); 6],
                [BitBoard(0); 6]
            ],
            player_turn: Colour::White,
            can_castle: [[true, true], [true, true]],
        }
    }

    pub fn reverse_move(&mut self, mv: AntiMove) {
        if let Some(captured) = mv.captured {
            let board = self.get_piece_board(self.player_turn, captured);
            self.update_bitboard(self.player_turn, captured, board | mv.from.to_bitboard());
        }
        self.player_turn = self.player_turn.other();
        self.can_castle[self.player_turn as usize][CastleSide::King as usize] = mv.kingside_castle;
        self.can_castle[self.player_turn as usize][CastleSide::Queen as usize] = mv.queenside_castle;

        let board = self.get_piece_board(self.player_turn, mv.piece);
        self.update_bitboard(self.player_turn, mv.piece, (board | mv.to.to_bitboard()) & !mv.from.to_bitboard());
    }

    pub fn make_reversible_move(&mut self, mv: Move) -> AntiMove {
        match mv {
            Move::Normal(mv) => self.make_reversible_normal_move(mv),
            Move::Castle(side, colour) => todo!()
        }
    }

    fn make_reversible_normal_move(&mut self, mv: NormalMove) -> AntiMove {
        let from = mv.from.to_bitboard();
        let to = mv.to.to_bitboard();

        let opponent = self.player_turn.other();
        let kingside_castle = self.can_castle(self.player_turn, CastleSide::King);
        let queenside_castle = self.can_castle(self.player_turn, CastleSide::Queen);

        let mut board = self.get_current_turn_boards()[mv.piece as usize];
        board = (board & !from) | to;
        self.update_bitboard(self.player_turn, mv.piece, board);
        self.player_turn = self.player_turn.other();

        if mv.piece == Piece::King {
            self.can_castle[self.player_turn as usize][CastleSide::King as usize] = false;
            self.can_castle[self.player_turn as usize][CastleSide::Queen as usize] = false;
        }
        else if mv.piece == Piece::Rook {
            let rook_board = self.get_piece_board(self.player_turn, Piece::Rook);
            match self.player_turn {
                Colour::White => {
                    if (rook_board & Square::A1.to_bitboard()).0 != 0 {
                        self.can_castle[self.player_turn as usize][CastleSide::King as usize] = false;
                    }
                    else if (rook_board & Square::H1.to_bitboard()).0 != 0 {
                        self.can_castle[self.player_turn as usize][CastleSide::Queen as usize] = false;
                    }
                }
                Colour::Black => {
                    if (rook_board & Square::A8.to_bitboard()).0 != 0 {
                        self.can_castle[self.player_turn as usize][CastleSide::King as usize] = false;
                    }
                    else if (rook_board & Square::H8.to_bitboard()).0 != 0 {
                        self.can_castle[self.player_turn as usize][CastleSide::Queen as usize] = false;
                    }
                }
            }

        }

        let mut captured = None;
        for piece in Piece::ALL {
            if self.get_piece(opponent, *piece).intersects(to) {
                let b = self.get_piece_board(opponent, *piece);
                self.update_bitboard(opponent, *piece, b & !to);
                captured = Some(piece.clone());
            }
        }
        AntiMove {
            from: mv.to,
            to: mv.from,
            piece: mv.piece,
            captured,
            kingside_castle,
            queenside_castle,
        }
    }

    pub fn get_piece(&self, colour: Colour, piece: Piece) -> BitBoard {
        self.piece_boards[colour as usize][piece as usize]
    }

    pub fn make_move(&self, mv: Move) -> Position {
        match mv {
            Move::Normal(normal) => self.make_normal_move(&normal), //TODO we have no validation for user input moves
            Move::Castle(side, ..) => self.make_castle_move(side)
        }
    }

    pub fn make_castle_move(&self, side: CastleSide) -> Position {
        let mut new_pos = self.clone();
        new_pos.player_turn = self.player_turn.other();
        new_pos.can_castle[self.player_turn as usize][CastleSide::King as usize] = false;
        new_pos.can_castle[self.player_turn as usize][CastleSide::Queen as usize] = false;

        let current_rooks = new_pos.piece_boards[self.player_turn as usize][Piece::Rook as usize];

        let (rook_start, rook_end, king_end) = match (self.player_turn, side) {
            (Colour::White, CastleSide::King) => (Square::H1, Square::F1, Square::G1),
            (Colour::White, CastleSide::Queen) => (Square::A1, Square::D1, Square::C1),
            (Colour::Black, CastleSide::King) => (Square::H8, Square::F8, Square::G8),
            (Colour::Black, CastleSide::Queen) => (Square::A8, Square::D8, Square::C8),
        };

        new_pos.piece_boards[self.player_turn as usize][Piece::Rook as usize] = (current_rooks & !rook_start.to_bitboard()) | rook_end.to_bitboard();
        new_pos.piece_boards[self.player_turn as usize][Piece::King as usize] = king_end.to_bitboard();

        new_pos
    }

    pub fn make_normal_move(&self, mv: &NormalMove) -> Position {
        let from = mv.from.to_bitboard();
        let to = mv.to.to_bitboard();

        let mut board = self.get_current_turn_boards()[ mv.piece as usize].clone();
        board = (board & !from) | to;
        let mut new_pos = self.clone();
        new_pos.update_bitboard(self.player_turn, mv.piece, board);
        new_pos.player_turn = self.player_turn.other();

        if mv.piece == Piece::King {
            new_pos.can_castle[self.player_turn as usize][CastleSide::King as usize] = false;
            new_pos.can_castle[self.player_turn as usize][CastleSide::Queen as usize] = false;
        }
        else if mv.piece == Piece::Rook {
            let rook_board = self.get_piece_board(self.player_turn, Piece::Rook);
            match self.player_turn {
                Colour::White => {
                    if (rook_board & Square::A1.to_bitboard()).0 != 0 {
                        new_pos.can_castle[self.player_turn as usize][CastleSide::King as usize] = false;
                    }
                    else if (rook_board & Square::H1.to_bitboard()).0 != 0 {
                        new_pos.can_castle[self.player_turn as usize][CastleSide::Queen as usize] = false;
                    }
                }
                Colour::Black => {
                    if (rook_board & Square::A8.to_bitboard()).0 != 0 {
                        new_pos.can_castle[self.player_turn as usize][CastleSide::King as usize] = false;
                    }
                    else if (rook_board & Square::H8.to_bitboard()).0 != 0 {
                        new_pos.can_castle[self.player_turn as usize][CastleSide::Queen as usize] = false;
                    }
                }
            }

        }

        let opponent = self.player_turn.other() as usize;
        for i in 0..6 {
            new_pos.piece_boards[opponent][i] = new_pos.piece_boards[opponent][i] & !to;
        }
        new_pos
    }

    fn get_current_turn_boards(&self) -> [BitBoard; 6] {
        self.piece_boards[self.player_turn as usize]
    }

    pub fn get_sides_pieces(&self, colour: Colour) -> BitBoard {
        let boards = self.piece_boards[colour as usize];
        boards.iter().cloned().reduce(|a,b| a | b).unwrap()
    }

    pub fn get_all_pieces(&self) -> BitBoard {
        self.piece_boards.iter().flatten().copied().reduce(|a,b| a | b).unwrap()
    }

    pub fn update_bitboard(&mut self, colour: Colour, piece: Piece, value: BitBoard) {
        self.piece_boards[colour as usize][piece as usize] = value;
    }

    pub fn get_piece_board(&self, colour: Colour, piece: Piece) -> BitBoard {
        self.piece_boards[colour as usize][piece as usize]
    }

    pub fn can_castle(&self, colour: Colour, side: CastleSide) -> bool {
        if !self.can_castle[colour as usize][side as usize] {
            return false;
        }

        let enemy_attacks = self.get_attacks(colour.other());
        let blockers = self.get_all_pieces();
        let castle_path = CASTLE_PATHS[colour as usize][side as usize];
        let king = self.get_piece_board(colour, Piece::King);

        if ((castle_path & blockers) | ((castle_path | king) & enemy_attacks)).0 != 0 {
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

    pub fn get_moves(&self, colour: Colour, piece: Piece) -> Option<BitBoard> {
        let board = self.get_piece(colour, piece);

        board.into_iter()
            .map(|sq| self.get_pseudo_moves_single_piece(colour, piece, sq))
            .map(|pseudo_moves| pseudo_moves & !self.get_sides_pieces(self.player_turn))
            .reduce(|a,b| a | b)
    }

    pub fn get_attacks(&self, colour: Colour) -> BitBoard {
        let attacks = self.get_moves(colour, Piece::Knight).unwrap_or(BitBoard(0))
            | self.get_moves(colour, Piece::Bishop).unwrap_or(BitBoard(0))
            | self.get_moves(colour, Piece::Rook).unwrap_or(BitBoard(0))
            | self.get_moves(colour, Piece::Queen).unwrap_or(BitBoard(0))
            | self.get_moves(colour, Piece::King).unwrap_or(BitBoard(0));

        attacks | all_pawn_attacks(self, colour)
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


#[cfg(test)]
mod tests {
    use super::*;
    use crate::pieces::Piece::{Bishop, Knight, Pawn, Queen};

    /// Plays normal moves from the starting position, panicking if any of them fail.
    fn play(moves: &[(Piece, Square, Square)]) -> Position {
        let mut pos = Position::new_standard_game();
        for &(piece, from, to) in moves {
            pos = pos
                .make_move(Move::Normal(NormalMove { piece, from, to }));
        }
        pos
    }

    fn castle(pos: &Position, side: CastleSide, colour: Colour) -> Position {
        pos.make_move(Move::Castle(side, colour))
    }

    fn assert_squares(pos: &Position, colour: Colour, piece: Piece, squares: &[Square]) {
        let expected = squares.iter().fold(0u64, |acc, sq| acc | sq.to_bitboard().0);
        let actual = pos.get_piece(colour, piece).0;
        assert_eq!(actual, expected, "{:?} board: got {:#018x}, expected {:#018x}", piece, actual, expected);
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
        let pos = castle(&play(&KINGSIDE_OPENING), CastleSide::King, Colour::White);

        assert_squares(&pos, Colour::White, Piece::King, &[Square::G1]);
        assert_squares(&pos, Colour::White, Piece::Rook, &[Square::A1, Square::F1]);
        assert!(!pos.can_castle[Colour::White as usize][CastleSide::King as usize]);
        assert!(!pos.can_castle[Colour::White as usize][CastleSide::Queen as usize]);
    }

    #[test]
    fn black_castles_kingside() {
        let mut moves = KINGSIDE_OPENING.to_vec();
        moves.push((Pawn, Square::D2, Square::D3));
        let pos = castle(&play(&moves), CastleSide::King, Colour::Black);

        assert_squares(&pos, Colour::Black, Piece::King, &[Square::G8]);
        assert_squares(&pos, Colour::Black, Piece::Rook, &[Square::A8, Square::F8]);
    }

    #[test]
    fn white_castles_queenside() {
        let pos = castle(&play(&QUEENSIDE_OPENING), CastleSide::Queen, Colour::White);

        assert_squares(&pos, Colour::White, Piece::King, &[Square::C1]);
        assert_squares(&pos, Colour::White, Piece::Rook, &[Square::D1, Square::H1]);
    }

    #[test]
    fn black_castles_queenside() {
        let mut moves = QUEENSIDE_OPENING.to_vec();
        moves.push((Pawn, Square::E2, Square::E3));
        let pos = castle(&play(&moves), CastleSide::Queen, Colour::Black);

        assert_squares(&pos, Colour::Black, Piece::King, &[Square::C8]);
        assert_squares(&pos, Colour::Black, Piece::Rook, &[Square::D8, Square::H8]);
    }

    #[test]
    fn castling_blocked_in_starting_position() {
        let pos = Position::new_standard_game();

        assert_eq!(pos.can_castle(Colour::White, CastleSide::King), false);
        assert_eq!(pos.can_castle(Colour::White, CastleSide::Queen), false);
    }

}
