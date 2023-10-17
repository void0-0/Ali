use crate::combo::format::Format;

pub trait ToFormat {
    fn to_format(&self, format: &Format) -> String;
}