#[derive(Copy, Clone)]
pub enum PunchTarget {
    Head,
    Body
}

impl PunchTarget {
    pub fn as_str(&self) -> &'static str {
        match self {
            PunchTarget::Head => "h",
            _ => "b"
        }
    }
}

impl From<u32> for PunchTarget {
    fn from(value: u32) -> Self {
        match value % 2 {
            0 => PunchTarget::Head,
            1 => PunchTarget::Body,
            _ => unreachable!("value % 2 should always return an int within 0 & 1")
        }
    }
}