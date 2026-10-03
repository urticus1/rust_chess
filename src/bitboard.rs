use std::ops::{BitAnd, BitOr, BitXor, Not};
use crate::{COL_A, COL_H, ROW_1, ROW_8, ROW_MASK};
use crate::moves::Direction;

#[derive(Clone)]
pub struct BitBoard {
    pub board: u64
}

impl BitBoard {
    pub fn print(&self) {
        print!("\n\n\n");
        for i in 0..8 {
            let mask = (ROW_MASK << 56) >> i * 8;
            print!("{} ", 7 - i + 1);
            let row = (self.board & mask) >> (56 - i * 8);
            self.print_row(row as u8);
        }
        print!("  ");
        for i in 0..8 {
            print!("{} ", ('A' as u8 + i) as char);
        }
        print!("\n\n\n");
    }

    fn print_row(&self, row: u8) {
        // println!("debug {}", row);
        for i in 0..8 {
            let mask = 0b_00000001 << i;
            let occupied = row & mask != 0;
            print!("{} ", if occupied { "X" } else { "." });
        }

        println!("");
    }

    pub fn shift(&self, direction: Direction) -> BitBoard {
        return match direction {
            Direction::Up => BitBoard { board: self.board << 8 },
            Direction::Down => BitBoard { board: self.board >> 8 },
            Direction::Left => BitBoard { board: self.board >> 1 & !COL_H.board },
            Direction::Right => BitBoard { board: self.board << 1 & !COL_A.board },
            Direction::LeftUp => { BitBoard { board: (self.board << 7) & !ROW_1.board & !COL_H.board} },
            Direction::LeftDown => { BitBoard { board: (self.board >> 9) & !ROW_8.board & !COL_H.board } },
            Direction::RightUp => { BitBoard { board: (self.board << 9) & ! ROW_1.board & !COL_A.board} },
            Direction::RightDown => { BitBoard { board: (self.board >> 7) & !ROW_8.board & !COL_A.board } },
        }
    }
}

impl BitAnd for BitBoard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            board: self.board & rhs.board
        }
    }
}

impl BitOr for BitBoard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            board: self.board | rhs.board
        }
    }
}


impl BitOr for &BitBoard {
    type Output = BitBoard;

    fn bitor(self, rhs: Self) -> Self::Output {
        BitBoard {
            board: self.board | rhs.board
        }
    }
}

impl Not for BitBoard {
    type Output = Self;

    fn not(self) -> Self::Output {
        return BitBoard { board: !self.board }
    }
}

impl Not for &BitBoard {
    type Output = BitBoard;

    fn not(self) -> Self::Output {
         BitBoard { board: !&self.board }
    }
}

impl BitAnd for &BitBoard {
    type Output = BitBoard;

    fn bitand(self, rhs: Self) -> Self::Output {
        BitBoard {
            board: self.board & rhs.board
        }
    }

}

impl BitXor for &BitBoard {
    type Output = BitBoard;

    fn bitxor(self, rhs: Self) -> Self::Output {
        BitBoard {
            board: self.board ^ rhs.board
        }
    }
}
impl BitXor for BitBoard {
    type Output = BitBoard;

    fn bitxor(self, rhs: Self) -> Self::Output {
        BitBoard {
            board: self.board ^ rhs.board
        }
    }
}