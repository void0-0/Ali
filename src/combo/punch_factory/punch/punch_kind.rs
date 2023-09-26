#[derive(Copy, Clone)]
pub enum PunchKind {
    Jab = 1,
    Direct = 2,
    LeftHook = 3,
    RightHook = 4,
    LeftUppercut = 5,
    RightUppercut = 6
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