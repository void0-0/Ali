use serde_derive::Deserialize;
use crate::config::side::Side;

#[derive(Deserialize)]
#[derive(Copy, Clone)]
pub enum Handedness {
    RightHanded,
    LeftHanded
}

impl Into<Side> for Handedness {
    fn into(self) -> Side {
        match self {
            Handedness::RightHanded => Side::Right,
            _ => Side::Left
        }
    }
}

