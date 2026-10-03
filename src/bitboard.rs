use std::ops::{BitAnd, BitOr, BitXor, Not, Shl};
use crate::{COL_A, COL_H, ROW_1, ROW_8, ROW_MASK};
use crate::moves::Direction;

#[derive(Copy, Clone)]
pub struct BitBoard(pub u64);

impl BitBoard {
    pub fn print(&self) {
        print!("\n\n\n");
        for i in 0..8 {
            let mask = (ROW_MASK << 56) >> i * 8;
            print!("{} ", 7 - i + 1);
            let row = (self.0 & mask) >> (56 - i * 8);
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
        return BitBoard(!self.0)
    }
}

impl Not for &BitBoard {
    type Output = BitBoard;

    fn not(self) -> Self::Output {
        BitBoard(!&self.0)
    }
}

    impl BitAnd for &BitBoard {
    type Output = BitBoard;

    fn bitand(self, rhs: Self) -> Self::Output {
        BitBoard (self.0 & rhs.0)
    }

}

impl BitXor for &BitBoard {
    type Output = BitBoard;

    fn bitxor(self, rhs: Self) -> Self::Output {
        BitBoard (self.0 ^ rhs.0)
    }
}
impl BitXor for BitBoard {
    type Output = BitBoard;

    fn bitxor(self, rhs: Self) -> Self::Output {
        BitBoard (self.0 ^ rhs.0)
    }
}