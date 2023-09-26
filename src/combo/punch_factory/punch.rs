mod punch_target;
mod punch_kind;
pub mod to_boxer_format;

use punch_target::PunchTarget;
use punch_kind::PunchKind;
use to_boxer_format::ToBoxerFormat;

#[derive(Copy, Clone)]
pub struct Punch {
    pub kind: PunchKind,
    pub target: PunchTarget
}

impl ToBoxerFormat for Punch {
    fn to_boxer_format(&self) -> String {
        format!("{}{}", self.kind as i32, self.target.as_str())
    }
}
