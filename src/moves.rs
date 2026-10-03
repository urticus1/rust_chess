use crate::bitboard::{BitBoard};

#[derive(Clone)]
pub struct Square {
    pub column: Column,
    pub row: Row
}


impl TryFrom<BitBoard> for Square {
    type Error = ();

    fn try_from(value: BitBoard) -> Result<Self, Self::Error> {
        if value.board == 0 {
            return Err(());
        }
        let index = value.board.trailing_zeros();
        let row = index / 8;
        let column = (value.board >> row * 8).trailing_zeros();
        Ok(Square {
            column: Column::from(column as usize),
            row: Row::from(row as usize)
        })
    }
}

impl TryFrom<&BitBoard> for Square {
    type Error = ();

    fn try_from(value: &BitBoard) -> Result<Self, Self::Error> {
        let index = value.board;
        let row = index / 7;
        let column = index >> row;
        Ok(Square {
            column: Column::from(column as usize + 1),
            row: Row::from(row as usize + 1)
        })
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

#[derive(Copy, Clone, PartialEq)]
pub enum Row {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}


#[derive(Copy, Clone, PartialEq)]
pub enum Column {
    A, B, C, D, E, F, G, H
}

impl Square {
    pub fn to_bitboard(&self) -> BitBoard {
        let row =  match &self.row {
            Row::One => 0,
            Row::Two => 1,
            Row::Three => 2,
            Row::Four => 3,
            Row::Five => 4,
            Row::Six => 5,
            Row::Seven => 6,
            Row::Eight => 7,
        };

        let column = match &self.column {
            Column::A => 0,
            Column::B => 1,
            Column::C => 2,
            Column::D => 3,
            Column::E => 4,
            Column::F => 5,
            Column::G => 6,
            Column::H => 7,
        };

        BitBoard {
            board: (1 << column) << 8 * row
        }
    }
}

impl Square {
    pub fn get_neighbour(&self, direction: &Direction) -> Option<Square> {
        match direction {
            Direction::Up => {
                if self.row == Row::Eight {
                    None
                } else {
                    let mut index: usize = self.row.into();
                    index = index + 1;
                    Some(Square { row: index.into(), column: self.column })
                }
            },
            Direction::Down => {
                if self.row == Row::One {
                    None
                } else {
                    let mut index: usize = self.row.into();
                    index = index - 1;
                    Some(Square { row: index.into(), column: self.column })
                }
            },
            Direction::Left => {
                if self.column == Column::A {
                    None
                } else {
                    let mut index: usize = self.column.into();
                    index = index - 1;
                    Some(Square { row: self.row, column: index.into() })
                }
            },
            Direction::Right => {
                if self.column == Column::H {
                    None
                } else {
                    let mut index: usize = self.column.into();
                    index = index + 1;
                    Some(Square { row: self.row, column: index.into() })
                }
            },
            Direction::LeftUp => {
                if self.column == Column::A || self.row == Row::Eight {
                    None
                } else {
                    let mut col_index: usize = self.column.into();
                    let mut row_index: usize = self.row.into();
                    col_index = col_index - 1;
                    row_index = row_index + 1;
                    Some(Square { row: row_index.into(), column: col_index.into() })
                }
            },
            Direction::LeftDown => {
                if self.column == Column::A || self.row == Row::One {
                    None
                } else {
                    let mut col_index: usize = self.column.into();
                    let mut row_index: usize = self.row.into();
                    col_index = col_index - 1;
                    row_index = row_index - 1;
                    Some(Square { row: row_index.into(), column: col_index.into() })
                }
            },
            Direction::RightUp => {
                if self.column == Column::H || self.row == Row::Eight {
                    None
                } else {
                    let mut col_index: usize = self.column.into();
                    let mut row_index: usize = self.row.into();
                    col_index = col_index + 1;
                    row_index = row_index + 1;
                    Some(Square { row: row_index.into(), column: col_index.into() })
                }
            },
            Direction::RightDown => {
                if self.column == Column::H || self.row == Row::One {
                    None
                } else {
                    let mut col_index: usize = self.column.into();
                    let mut row_index: usize = self.row.into();
                    col_index = col_index + 1;
                    row_index = row_index - 1;
                    Some(Square { row: row_index.into(), column: col_index.into() })
                }
            },
        }
    }
}

impl Into<usize> for Column {
    fn into(self) -> usize {
        match self {
            Column::A => 0,
            Column::B => 1,
            Column::C => 2,
            Column::D => 3,
            Column::E => 4,
            Column::F => 5,
            Column::G => 6,
            Column::H => 7,
        }
    }
}

impl From<usize> for Column {
    fn from(value: usize) -> Column {
        match value {
            0 => Column::A,
            1 => Column::B,
            2 => Column::C,
            3 => Column::D,
            4 => Column::E,
            5 => Column::F,
            6 => Column::G,
            7 => Column::H,
            _ => panic!("Invalid column value {}", value)
        }
    }
}

impl Into<usize> for Row {
    fn into(self) -> usize {
        match self {
            Row::One => 0,
            Row::Two => 1,
            Row::Three => 2,
            Row::Four => 3,
            Row::Five => 4,
            Row::Six => 5,
            Row::Seven => 6,
            Row::Eight => 7,
        }
    }
}

impl From<usize> for Row {
    fn from(value: usize) -> Self {
        match value {
            0 => Row::One,
            1 => Row::Two,
            2 => Row::Three,
            3 => Row::Four,
            4 => Row::Five,
            5 => Row::Six,
            6 => Row::Seven,
            7 => Row::Eight,
            _ => panic!("illegal row value: {}", value)
        }
    }
}
