use rand::Rng;

#[derive(Copy, Clone)]
pub enum PunchKind {
    Jab = 1,
    Direct = 2,
    LeftHook = 3,
    RightHook = 4,
    LeftUppercut = 5,
    RightUppercut = 6
}

impl PunchKind {
    pub fn gen_left_side_punch_kind() -> PunchKind {
        let mut rng = rand::thread_rng();
        let left_side_punch_kinds = vec![PunchKind::Jab, PunchKind::LeftHook, PunchKind::LeftUppercut];
        left_side_punch_kinds[rng.gen_range(0..=2)]
    }

    pub fn gen_right_side_punch_kind() -> PunchKind {
        let mut rng = rand::thread_rng();
        let left_side_punch_kinds = vec![PunchKind::Direct, PunchKind::RightHook, PunchKind::RightUppercut];
        left_side_punch_kinds[rng.gen_range(0..=2)]
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