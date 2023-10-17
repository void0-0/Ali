use rand::RngCore;
use crate::combo::format::Format;
use crate::combo::ToFormat;

#[derive(Copy, Clone)]
pub enum PunchTarget {
    Head,
    Body
}

impl PunchTarget {
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

impl ToFormat for PunchTarget {
    fn to_format(&self, format: &Format) -> String {
        String::from(
            match format {
                Format::Normal =>
                    match self {
                        PunchTarget::Head => "Head",
                        PunchTarget::Body => "Body"
                    },
                Format::Boxer =>
                    match self {
                        PunchTarget::Head => "h",
                        PunchTarget::Body => "b"
                    }
            }
        )
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