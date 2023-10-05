use serde_derive::Deserialize;
use crate::config::side::Side;

#[derive(Deserialize)]
#[derive(Copy, Clone)]
pub enum Stance {
    Orthodox,
    Southpaw
}

impl Into<Side> for Stance {
    fn into(self) -> Side {
        match self {
            Stance::Orthodox => Side::Right,
            _ => Side::Left
        }
    }
}

