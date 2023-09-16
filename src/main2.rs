use std::env::{self, Args};
use rand::prelude::*;

// Add more readable format
// Add distance (step in and step out)
// Add Config file reading
// Add allow same side hits
// Add custom separator

#[derive(Copy, Clone)]
enum PunchKind {
    Jab = 1,
    Direct = 2,
    LeftHook = 3,
    RightHook = 4,
    LeftUppercut = 5,
    RightUppercut = 6
}

// Could be deried using mun_trait & num_derive
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

#[derive(Copy, Clone)]
enum PunchTarget {
    Head,
    Body
}

impl PunchTarget {
    fn as_str(&self) -> &'static str {
        match self {
            PunchTarget::Head => "h",
            _ => "b"
        }
    }
}

// Could be deried using mun_trait & num_derive
impl From<u32> for PunchTarget {
    fn from(value: u32) -> Self {
        match value % 2 {
            0 => PunchTarget::Head,
            1 => PunchTarget::Body,
            _ => unreachable!("value % 2 should always return an int within 0 & 1")
        }
    }
}

pub struct PunchFactory {
    state: bool
}

impl PunchFactory {
    pub fn new() -> PunchFactory {
        PunchFactory{ state: true }
    }
    
    pub fn generate_punch(&mut self) -> Punch {
        let mut rng = rand::thread_rng();
        
        let kind = if self.state {
            rng.gen_range(1..=3).into()
        } else {
            rng.gen_range(3..=5).into()
        };
        
        let target = rng.next_u32().into();
        
        self.state = !self.state;
        
        Punch { kind , target }
    }
}

trait ToBoxerFormat {
    fn to_boxer_format(&self) -> String;
}

#[derive(Copy, Clone)]
pub struct Punch {
    kind: PunchKind,
    target: PunchTarget
}

impl ToBoxerFormat for Punch {
    fn to_boxer_format(&self) -> String {
        format!("{}{}", self.kind as i32, self.target.as_str())
    }
}

pub struct Combo {
    inner: Vec<Punch>
}

impl Combo {
    pub fn new(length: i32) -> Combo {
        let mut factory = PunchFactory::new();
        
        Combo { inner: (0..length).map(|_| factory.generate_punch()).collect::<Vec<_>>() }
    }
}

impl ToBoxerFormat for Combo {
    fn to_boxer_format(&self) -> String {
        self.inner.iter().map(|punch| punch.to_boxer_format()).collect::<Vec<String>>().join(", ")
    }
}

const DEFAULT_COMBO_LENGTH: i32 = 4;

fn main() {
    let length = parse_length(env::args()).unwrap_or(DEFAULT_COMBO_LENGTH);
    let combo = Combo::new(length);
    
    println!("{}", combo.to_boxer_format());
}

fn parse_length(mut raw_args: Args) -> Option<i32> {
    raw_args.skip(1).next().and_then(|first_arg| first_arg.parse::<i32>().ok())
}