pub mod punch_target;
pub mod punch_kind;
pub mod to_format;

use colored::Colorize;
use punch_target::PunchTarget;
use punch_kind::PunchKind;
use to_format::ToFormat;
use crate::combo::format::Format;

#[derive(Copy, Clone)]
pub struct Punch {
    pub kind: PunchKind,
    pub target: PunchTarget
}

impl ToFormat for Punch {
    fn to_format(&self, format: &Format) -> String {
        format!(
            "{}{}{}",
            self.kind.to_format(format).bright_cyan(),
            match format {
                Format::Normal => " ",
                Format::Boxer => ""
            },
            self.target.to_format(format).bright_yellow()
        )
    }
}
