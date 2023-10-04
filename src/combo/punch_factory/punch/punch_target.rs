use rand::RngCore;

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

    pub fn gen_target(allow_head_punch: bool, allow_body_punch: bool) -> PunchTarget {
        let mut rng = rand::thread_rng();

        match allow_head_punch && allow_body_punch {
            true => rng.next_u32().into(),
            false => match allow_head_punch {
                true => PunchTarget::Head,
                false => PunchTarget::Body
            }
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