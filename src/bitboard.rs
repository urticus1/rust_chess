use std::fmt;
use std::ops::{BitAnd, BitOr, BitXor, Not, Shl};
use crate::{COL_A, COL_H, ROW_1, ROW_8, ROW_MASK};
use crate::squares::{Direction, Square};

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct BitBoard(pub u64);

impl fmt::Display for BitBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n\n\n")?;
        for i in 0..8 {
            let mask = (ROW_MASK << 56) >> i * 8;
            write!(f, "{} ", 7 - i + 1)?;
            let row = (self.0 & mask) >> (56 - i * 8);
            self.fmt_row(f, row as u8)?;
        }
        write!(f, "  ")?;
        for i in 0..8 {
            write!(f, "{} ", ('A' as u8 + i) as char)?;
        }
        write!(f, "\n\n\n")
    }
}

impl Iterator for BitBoard {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }
        let zeros = self.0.trailing_zeros();
        self.0 = self.0 & !(1 << zeros);
        Some(Square::try_from(zeros as u64).expect("bitboard iterator generated non unique sqaure"))
    }
}

impl BitBoard {
    fn fmt_row(&self, f: &mut fmt::Formatter, row: u8) -> fmt::Result {
        for i in 0..8 {
            let mask = 0b_00000001 << i;
            let occupied = row & mask != 0;
            write!(f, "{} ", if occupied { "X" } else { "." })?;
        }

        writeln!(f)
    }

    pub const fn shift(&self, direction: Direction) -> BitBoard {
        match direction {
            Direction::Up => BitBoard(self.0 << 8),
            Direction::Down => BitBoard(self.0 >> 8),
            Direction::Left => BitBoard(self.0 >> 1 & !COL_H.0),
            Direction::Right => BitBoard(self.0 << 1 & !COL_A.0),
            Direction::LeftUp =>  BitBoard((self.0 << 7) & !ROW_1.0 & !COL_H.0),
            Direction::LeftDown => BitBoard((self.0 >> 9) & !ROW_8.0 & !COL_H.0),
            Direction::RightUp => BitBoard((self.0 << 9) & ! ROW_1.0 & !COL_A.0),
            Direction::RightDown =>  BitBoard((self.0 >> 7) & !ROW_8.0 & !COL_A.0),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    pub fn intersects(&self, other: BitBoard) -> bool {
        self.0 & other.0 != 0
    }

    pub fn count_ones(&self) -> usize {
        self.0.count_ones() as usize
    }
}

impl BitAnd for BitBoard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self (self.0 & rhs.0)
    }
}

impl BitOr for BitBoard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self (self.0 | rhs.0)
    }
}


impl BitOr for &BitBoard {
    type Output = BitBoard;

    fn bitor(self, rhs: Self) -> Self::Output {
        BitBoard (self.0 | rhs.0)
    }
}

impl Not for BitBoard {
    type Output = Self;

    fn not(self) -> Self::Output {
        BitBoard(!self.0)
    }
}


impl BitXor for BitBoard {
    type Output = BitBoard;

    fn bitxor(self, rhs: Self) -> Self::Output {
        BitBoard (self.0 ^ rhs.0)
    }
}