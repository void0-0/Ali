use serde_derive::Deserialize;

#[derive(Deserialize)]
pub enum Handedness {
    RightHanded,
    LeftHanded
}

impl Handedness {
    pub fn as_str(&self) -> &'static str {
        match self {
            Handedness::RightHanded => "right_handed",
            _ => "left_handed"
        }
    }
}

