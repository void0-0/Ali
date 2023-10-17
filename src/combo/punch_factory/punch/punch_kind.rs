use num_derive::FromPrimitive;
use rand::Rng;
use crate::combo::format::Format;
use crate::combo::ToFormat;
use crate::config::side::Side;

#[derive(Copy, Clone, FromPrimitive)]
pub enum PunchKind {
    Jab = 1,
    Direct = 2,
    LeftHook = 3,
    RightHook = 4,
    LeftUppercut = 5,
    RightUppercut = 6
}

const RIGHT_SIDE_PUNCH_KINDS: [PunchKind; 3] = [PunchKind::Direct, PunchKind::RightHook, PunchKind::RightUppercut];
const LEFT_SIDE_PUNCH_KINDS: [PunchKind; 3] = [PunchKind::Jab, PunchKind::LeftHook, PunchKind::LeftUppercut];

impl PunchKind {
    pub fn gen_punch_kind(is_first_punch: bool, ready_side: Side) -> PunchKind {
        match is_first_punch {
            true => {
                let mut possible_punch_kinds = match ready_side {
                    Side::Right => RIGHT_SIDE_PUNCH_KINDS.to_vec(),
                    Side::Left => LEFT_SIDE_PUNCH_KINDS.to_vec()
                };

                // Making sure to add PunchKind::Jab only once
                if ready_side == Side::Right {
                    possible_punch_kinds.push(PunchKind::Jab);
                }

                PunchKind::choose_from_vec(possible_punch_kinds)
            },
            false => match ready_side {
                Side::Right => PunchKind::choose_from_vec(RIGHT_SIDE_PUNCH_KINDS.to_vec()),
                Side::Left => PunchKind::choose_from_vec(LEFT_SIDE_PUNCH_KINDS.to_vec())
            }
        }
    }

    pub fn gen_random_punch_kind() -> PunchKind {
        let mut rng = rand::thread_rng();
        PunchKind::from(rng.gen_range(0..=6))
    }

    fn choose_from_vec(punch_kinds: Vec<PunchKind>) -> PunchKind {
        let mut rng = rand::thread_rng();
        punch_kinds[rng.gen_range(0..punch_kinds.len())]
    }
}

impl ToFormat for PunchKind {
    fn to_format(&self, format: &Format) -> String {
        match format {
            Format::Normal =>
                String::from( match self {
                    PunchKind::Jab => "Jab",
                    PunchKind::Direct => "Direct",
                    PunchKind::LeftHook => "Left hook",
                    PunchKind::RightHook => "Right hook",
                    PunchKind::LeftUppercut => "Left uppercut",
                    PunchKind::RightUppercut => "Right uppercut"
                }),
                Format::Boxer => (self.clone() as i32).to_string()
            }
    }
}

impl From<u32> for PunchKind {
    fn from(value: u32) -> Self {
        match value % 6 + 1 {
            1 => PunchKind::Jab,
            2 => PunchKind::Direct,
            3 => PunchKind::LeftHook,
            4 => PunchKind::RightHook,
            5 => PunchKind::LeftUppercut,
            6 => PunchKind::RightUppercut,
            _ => unreachable!("value % 6 + 1 should always return an int within 1 & 6")
        }
    }
}