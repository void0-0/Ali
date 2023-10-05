use rand::Rng;
use crate::config::side::Side;

#[derive(Copy, Clone)]
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