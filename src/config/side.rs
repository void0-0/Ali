use std::ops;

#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub enum Side {
    Right,
    Left
}

impl ops::Not for Side {
    type Output = Side;

    fn not(self) -> Self::Output {
        match self {
            Side::Right => Side::Left,
            _ => Side::Right
        }
    }
}