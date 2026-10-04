use std::fmt::format;
use crate::bitboard::{BitBoard};
use crate::{COL_A, COL_H};

macro_rules! squares {
    ($($sq:ident => $i:literal),* $(,)?) => {

        #[derive(Copy, Clone, PartialEq, Eq, Debug)]
        pub enum Square {$($sq),*}

        const ALL: &[Square; 64] = &[$(Square::$sq),*];

        impl TryFrom<usize> for Square {
            type Error = ();

            fn try_from(value: usize) -> Result<Self, Self::Error> {
                Ok(self::ALL[value])
            }
        }

        impl TryFrom<u64> for Square {
            type Error = (String);

            fn try_from(value: u64) -> Result<Self, Self::Error> {
                if value < 0 || value > 63 {
                    return Err((format!("index {} not valid for square", value)));
                }
                Ok(self::ALL[value as usize])
            }
        }

        impl TryFrom<String> for Square {
            type Error = ();
            fn try_from(value: String) -> Result<Self, Self::Error> {
                match value.as_str().to_uppercase().as_str() {
                    $(stringify! {$sq} => Ok(Square::$sq),)*
                    _ => Err(())
                }
            }
        }
    }
}

squares! {

    A1 => 0,
    B1 => 1,
    C1 => 2,
    D1 => 3,
    E1 => 4,
    F1 => 5,
    G1 => 6,
    H1 => 7,
    A2 => 8,
    B2 => 9,
    C2 => 10,
    D2 => 11,
    E2 => 12,
    F2 => 13,
    G2 => 14,
    H2 => 15,
    A3 => 16,
    B3 => 17,
    C3 => 18,
    D3 => 19,
    E3 => 20,
    F3 => 21,
    G3 => 22,
    H3 => 23,
    A4 => 24,
    B4 => 25,
    C4 => 26,
    D4 => 27,
    E4 => 28,
    F4 => 29,
    G4 => 30,
    H4 => 31,
    A5 => 32,
    B5 => 33,
    C5 => 34,
    D5 => 35,
    E5 => 36,
    F5 => 37,
    G5 => 38,
    H5 => 39,
    A6 => 40,
    B6 => 41,
    C6 => 42,
    D6 => 43,
    E6 => 44,
    F6 => 45,
    G6 => 46,
    H6 => 47,
    A7 => 48,
    B7 => 49,
    C7 => 50,
    D7 => 51,
    E7 => 52,
    F7 => 53,
    G7 => 54,
    H7 => 55,
    A8 => 56,
    B8 => 57,
    C8 => 58,
    D8 => 59,
    E8 => 60,
    F8 => 61,
    G8 => 62,
    H8 => 63,
}



impl TryFrom<BitBoard> for Square {
    type Error = ();

    fn try_from(value: BitBoard) -> Result<Self, Self::Error> {
        if value.0 == 0 || value.0.count_ones() != 1 {
            return Err(());
        }
        Ok(Square::try_from(value.0.trailing_zeros() as usize).expect("Invalid index for square"))
    }
}

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
    LeftUp,
    LeftDown,
    RightUp,
    RightDown,
}

impl Square {
    pub fn to_bitboard(self) -> BitBoard {
        BitBoard(1 << self as usize)
    }
}

impl Square {
    pub fn get_neighbour(self, direction: &Direction) -> Option<Square> {
        let index = self as u64;
        match direction {
            Direction::Up => {
                if index > 55 {
                    None
                } else {
                    Some(Square::try_from(index + 8).expect(format!("Tried to create square from index {}", index).as_str()))
                }
            },
            Direction::Down => {
                if index < 8 {
                    None
                } else {
                    Some(Square::try_from(index - 8).expect(format!("Tried to create square from index {}", index).as_str()))
                }
            },
            Direction::Left => {
                if (1 << index & COL_A.0) != 0 {
                    None
                } else {
                    Some(Square::try_from(index - 1).expect(format!("Tried to create square from index {}", index).as_str()))
                }
            },
            Direction::Right => {
                if (1 << index & COL_H.0) != 0 {
                    None
                } else {
                    Some(Square::try_from(index + 1).expect(format!("Tried to create square from index {}", index).as_str()))
                }
            },
            Direction::LeftUp => {
                if (1 << index & COL_A.0) != 0 || index > 55 {
                    None
                } else {
                    Some(Square::try_from(index + 7).expect(format!("Tried to create square from index {}", index).as_str()))
                }
            },
            Direction::LeftDown => {
                if (1 << index & COL_A.0) != 0 || index < 8 {
                    None
                } else {
                    Some(Square::try_from(index - 9).expect(format!("Tried to create square from index {}", index).as_str()))
                }
            },
            Direction::RightUp => {
                if (1 << index & COL_H.0) != 0 || index > 55 {
                    None
                } else {
                    Some(Square::try_from(index + 9).expect(format!("Tried to create square from index {}", index).as_str()))
                }
            },
            Direction::RightDown => {
                if (1 << index & COL_H.0) != 0 || index < 8 {
                    None
                } else {
                    Some(Square::try_from(index - 7).expect(format!("Tried to create square from index {}", index).as_str()))
                }
            },
        }
    }
}