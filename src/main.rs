use std::env::{self, Args};

// Add more readable format
// Add distance (step in and step out)
// Add Config file reading
    // Add allow same side hits
    // Add allow body hits
    // Add allow head hits
    // Add custom separator
    // Add lefty or righty boxer
    // Add DEFAULT_COMBO_LENGTH to config

mod combo;

use combo::Combo;
use combo::ToBoxerFormat;

const DEFAULT_COMBO_LENGTH: i32 = 4;

fn main() {
    let length = parse_length(env::args()).unwrap_or(DEFAULT_COMBO_LENGTH);
    let combo = Combo::new(length);

    println!("{}", combo.to_boxer_format());
}

fn parse_length(mut raw_args: Args) -> Option<i32> {
    raw_args.skip(1).next().and_then(|first_arg| first_arg.parse::<i32>().ok())
}