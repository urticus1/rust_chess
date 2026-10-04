use crate::bitboard::BitBoard;
use crate::game::{CastleSide, Move, MoveError, NormalMove};
use crate::moves::Direction::{LeftDown, LeftUp, RightDown, RightUp};
use crate::moves::Square;
use crate::pieces::{Colour, Piece};

const WHITE_KINGSIDE_CASTLE_PATH: BitBoard = BitBoard(0b01100000u64);
const WHITE_QUEENSIDE_CASTLE_PATH: BitBoard = BitBoard(0b00001110u64);

const BLACK_KINGSIDE_CASTLE_PATH: BitBoard = BitBoard(0b01100000u64 << (8 * 7));
const BLACK_QUEENSIDE_CASTLE_PATH: BitBoard = BitBoard(0b00001110u64 << (8 * 7));

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

impl Position {

    pub fn new_standard_game() -> Self {
        Position {
            piece_boards: [
                [
                    crate::game::STARTING_PAWNS_WHITE,
                    crate::game::STARTING_ROOKS_WHITE,
                    crate::game::STARTING_KNIGHTS_WHITE,
                    crate::game::STARTING_BISHOPS_WHITE,
                    crate::game::STARTING_QUEEN_WHITE,
                    crate::game::STARTING_KING_WHITE,
                ],
                [
                    crate::game::STARTING_PAWNS_BLACK,
                    crate::game::STARTING_ROOKS_BLACK,
                    crate::game::STARTING_KNIGHTS_BLACK,
                    crate::game::STARTING_BISHOPS_BLACK,
                    crate::game::STARTING_QUEEN_BLACK,
                    crate::game::STARTING_KING_BLACK,

                ]
            ],
            player_turn: Colour::White,
            can_castle: [[true, true], [true, true]],
        }


    }

    pub fn get_piece(&self, colour: Colour, piece: Piece) -> BitBoard {
        self.piece_boards[colour as usize][piece as usize]
    }

    pub fn make_move(&self, mv: Move) -> Result<Position, MoveError> {
        match mv {
            Move::Normal(normal) => self.make_normal_move(&normal),
            Move::Castle(side) => self.make_castle_move(side)
        }
    }

    pub fn make_castle_move(&self, side: CastleSide) -> Result<Position, MoveError> {
        if !self.can_castle[self.player_turn as usize][side as usize] {
            return Err(MoveError::new("Player has previously lost castle rights on this side"));
        }

        let enemy_attacks = self.get_attacks(self.player_turn.other());
        let blockers = self.get_all_pieces();
        let castle_path = CASTLE_PATHS[self.player_turn as usize][side as usize];
        let king = self.get_piece_board(self.player_turn, Piece::King);

        if ((castle_path & blockers) | ((castle_path | king) & enemy_attacks)).0 != 0 {
            return Err(MoveError::new("Castle is blocked"));
        }

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

        Ok(new_pos)
    }

    pub fn make_normal_move(&self, mv: &NormalMove) -> Result<Position, MoveError> {
        let from = mv.from.to_bitboard();
        let to = mv.to.to_bitboard();

        if (&from & &self.get_piece(self.player_turn, mv.piece)).0 == 0 {
            return Err(MoveError::new("No correct colour piece on square"));
        }

        let pseudo_moves = self.get_moves_single_piece(self.player_turn, mv.piece, &mv.from);
        if pseudo_moves.is_none() {
            return Err(MoveError::new("No correct position piece on square"));
        }
        let pseudo_moves = pseudo_moves.unwrap() & !self.get_sides_pieces(self.player_turn);


        if (&to & &pseudo_moves).0 == 0 {
            return Err(MoveError::new("Not a valid destination psuedo move"));
        }
        let index: usize = mv.piece as usize;

        let mut board = self.get_current_turn_boards()[index].clone();
        board = (board ^ from) | to;
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

        let opposition_attacks = new_pos.get_attacks(self.player_turn.other());
        if (opposition_attacks & new_pos.get_piece(self.player_turn, Piece::King)).0 != 0 {
            return Err(MoveError::new("King in check"));
        }
        Ok(new_pos)
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

    pub fn get_next_positions(&self) -> Vec<Position> {
        let pieces = self.get_pieces_and_squares(self.player_turn);

        pieces
            .into_iter()
            .map(
                |p| self.get_moves_single_piece(self.player_turn, p.0, &p.1)
                    .map(|moves| self.find_positions_from_move_set(p.0, &p.1, &moves))
                    .unwrap_or(vec![])
            )
            .flatten()
        .collect()
    }

    pub fn update_bitboard(&mut self, colour: Colour, piece: Piece, value: BitBoard) {
        self.piece_boards[colour as usize][piece as usize] = value;
    }

    pub fn get_piece_board(&self, colour: Colour, piece: Piece) -> BitBoard {
        self.piece_boards[colour as usize][piece as usize]
    }

    fn find_positions_from_move_set(&self, piece: Piece, piece_position: &Square, moves: &BitBoard) -> Vec<Position> {

        let mut remaining_moves = moves.clone().0;
        let mut next_moves = Vec::new();
        while remaining_moves != 0 {
            let zeros = remaining_moves.trailing_zeros();
            let to = BitBoard(1 << zeros);
            let piece_board = self.get_piece_board(self.player_turn, piece);
            let piece_position = piece_position.to_bitboard();
            let result = &piece_board & &!piece_position | to.clone();

            let mut next_pos = self.clone();
            next_pos.update_bitboard(self.player_turn, piece, result);


            let opponent = self.player_turn.other() as usize;
            for i in 0..6 {
                next_pos.piece_boards[opponent][i] = next_pos.piece_boards[opponent][i] & !to;
            }
            next_moves.push(next_pos);
            remaining_moves = remaining_moves & !&to.0;
        }
        next_moves
    }

   fn get_pieces_and_squares(&self, colour: Colour) -> Vec<(Piece, Square)> {
       let mut board = self.get_sides_pieces(colour).0;

       let mut squares = Vec::new();
       while board != 0 {
           let sq = board.trailing_zeros();
           let sq = BitBoard(1 << sq);
           let piece = self.piece_boards[colour as usize]
               .iter()
               .enumerate()
               .find(|b| (b.1 & &sq).0 != 0)
               .map(|b| b.0)
               .unwrap();
           board = board & board -1;
           let piece = match piece {
               0 => Piece::Pawn,
               1 => Piece::Rook,
               2 => Piece::Knight,
               3 => Piece::Bishop,
               4 => Piece::Queen,
               5 => Piece::King,
               _ => {
                   panic!("invalid piece nunmber");
               }
           };
           squares.push((piece, Square::try_from(sq).unwrap()));
       }
       squares
   }

    pub fn get_moves_single_piece(&self, colour: Colour, piece: Piece, square: &Square) -> Option<BitBoard> {
        let board = self.piece_boards[colour as usize][piece as usize];

        let pos = &square.to_bitboard() & &board;
        if pos.0 == 0 {
            return None;
        }
        let pseudo_moves = piece.get_pseudo_moves(&self, &pos.try_into().unwrap(), self.get_all_pieces(), colour);
        Some(pseudo_moves & ! self.get_sides_pieces(colour))
    }

    pub fn get_moves(&self, colour: Colour, piece: Piece) -> Option<BitBoard> {
        let squares = self.find_piece_squares(piece, colour);

        squares.into_iter()
            .map(|sq| self.get_moves_single_piece(colour, piece, &sq))
            .flatten()
            .reduce(|a,b| a | b)
    }

    pub fn find_piece_squares(&self, piece: Piece, colour: Colour) -> Vec<Square> {
        let mut board: u64 = self.get_piece(colour, piece).0;

        let mut result = vec![];
        while board != 0 {
            let lowest_occupied = board.trailing_zeros();
            let square = 1 << lowest_occupied;
            let square = BitBoard(square);

            result.push(Square::try_from(square).unwrap());
            board &= board - 1;
        }

        result
    }

    pub fn get_attacks(&self, colour: Colour) -> BitBoard {
        let attacks = self.get_moves(colour, Piece::Knight).unwrap_or(BitBoard(0))
            | self.get_moves(colour, Piece::Bishop).unwrap_or(BitBoard(0))
            | self.get_moves(colour, Piece::Rook).unwrap_or(BitBoard(0))
            | self.get_moves(colour, Piece::Queen).unwrap_or(BitBoard(0))
            | self.get_moves(colour, Piece::King).unwrap_or(BitBoard(0));

        attacks | self.get_pawn_attacks(colour)
    }

    pub fn get_pawn_attacks(&self, colour: Colour) -> BitBoard {
        let board = self.piece_boards[colour as usize][Piece::Pawn as usize];

        match colour {
            Colour::White => board.shift(LeftUp) | board.shift(RightUp),
            Colour::Black => board.shift(LeftDown) | board.shift(RightDown),
        }
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
                .make_move(Move::Normal(NormalMove { piece, from, to }))
                .unwrap_or_else(|e| panic!("setup move {:?} {:?} -> {:?} failed: {}", piece, from, to, e));
        }
        pos
    }

    fn castle(pos: &Position, side: CastleSide) -> Position {
        pos.make_move(Move::Castle(side))
            .unwrap_or_else(|e| panic!("{:?} castle failed: {}", side, e))
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
        let pos = castle(&play(&KINGSIDE_OPENING), CastleSide::King);

        assert_squares(&pos, Colour::White, Piece::King, &[Square::G1]);
        assert_squares(&pos, Colour::White, Piece::Rook, &[Square::A1, Square::F1]);
        assert!(!pos.can_castle[Colour::White as usize][CastleSide::King as usize]);
        assert!(!pos.can_castle[Colour::White as usize][CastleSide::Queen as usize]);
    }

    #[test]
    fn black_castles_kingside() {
        let mut moves = KINGSIDE_OPENING.to_vec();
        moves.push((Pawn, Square::D2, Square::D3));
        let pos = castle(&play(&moves), CastleSide::King);

        assert_squares(&pos, Colour::Black, Piece::King, &[Square::G8]);
        assert_squares(&pos, Colour::Black, Piece::Rook, &[Square::A8, Square::F8]);
    }

    #[test]
    fn white_castles_queenside() {
        let pos = castle(&play(&QUEENSIDE_OPENING), CastleSide::Queen);

        assert_squares(&pos, Colour::White, Piece::King, &[Square::C1]);
        assert_squares(&pos, Colour::White, Piece::Rook, &[Square::D1, Square::H1]);
    }

    #[test]
    fn black_castles_queenside() {
        let mut moves = QUEENSIDE_OPENING.to_vec();
        moves.push((Pawn, Square::E2, Square::E3));
        let pos = castle(&play(&moves), CastleSide::Queen);

        assert_squares(&pos, Colour::Black, Piece::King, &[Square::C8]);
        assert_squares(&pos, Colour::Black, Piece::Rook, &[Square::D8, Square::H8]);
    }

    #[test]
    fn castling_blocked_in_starting_position() {
        let pos = Position::new_standard_game();

        assert!(pos.make_move(Move::Castle(CastleSide::King)).is_err());
        assert!(pos.make_move(Move::Castle(CastleSide::Queen)).is_err());
    }
}
