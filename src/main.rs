use std::env::{self, Args};
use rand::Rng;

// Add more readable format
// Add distance (step in and step out)
// Add Config file reading
    // Add allow same side hits
    // Add custom seperator

#[derive(Copy, Clone)]
enum PunchKind {
    Jab = 1,
    Direct = 2,
    LeftHook = 3,
    RightHook = 4,
    LeftUppercut = 5,
    RightUppercut = 6
}

#[derive(Copy, Clone)]
enum PunchTarget {
    Head,
    Body
}

pub struct PunchFactory {
    state: bool
}

impl PunchFactory {
    const PUNCH1: &'static [PunchKind] = &[PunchKind::Direct, PunchKind::RightHook, PunchKind::RightUppercut];

    pub fn new() -> PunchFactory {
        PunchFactory{ state: true }
    }

    pub fn generate_punch(&mut self) -> Punch {
        let mut rng = rand::thread_rng();
        let punch_kind_index = rng.gen_range(0..=2);

        let kind = match self.state {
            true => Self::PUNCH1.get(punch_kind_index).unwrap_or(&PunchKind::Direct).to_owned(),
            false => vec![PunchKind::Jab, PunchKind::LeftHook, PunchKind::LeftUppercut][punch_kind_index]
        };

        let target = match rng.gen_range(0..=1) {
            0 => PunchTarget::Head,
            _ => PunchTarget::Body
        };

        self.state = !self.state;

        Punch { kind , target }
    }
}

#[derive(Copy, Clone)]
pub struct Punch {
    kind: PunchKind,
    target: PunchTarget
}

impl Punch {
    pub fn to_boxer_format(&self) -> String {
        let number = self.kind as i32;
        let target_code = match self.target {
            PunchTarget::Head => "h",
            _ => "b"
        };

        format!("{number}{target_code}")
    }
}

pub struct Combo {
    combo: Vec<Punch>
}

impl Combo {
    pub fn new(length: i32) -> Combo {
        let mut combo: Vec<Punch> = vec![];
        let mut factory = PunchFactory::new();

        for _ in 0..length {
            combo.push(factory.generate_punch());
        }

        Combo { combo }
    }

    pub fn to_boxer_format(&self, separator: &str) -> String {
        self.combo.iter().map(|punch| {
            punch.to_boxer_format()
        }).collect::<Vec<String>>().join(separator)
    }
}

fn main() {
    let length = parse_length(env::args());
    let combo = Combo::new(length);

    println!("{}", combo.to_boxer_format(", "));
}

fn parse_length(mut raw_args: Args) -> i32 {
    const DEFAULT_LENGTH: i32 = 4;

    let _ = raw_args.next();
    let args = raw_args.collect::<Vec<String>>();

    match args.first() {
        Some(first_arg) => first_arg.parse::<i32>().unwrap_or(DEFAULT_LENGTH),
        None => DEFAULT_LENGTH
    }
}