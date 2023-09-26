use std::env;

// Add more readable format
// Add distance (step in and step out)
    // Make sure you hit a couple of times in between each steps

mod combo;
mod config;

use combo::Combo;
use combo::ToBoxerFormat;
use config::Config;

const DEFAULT_COMBO_LENGTH: i32 = 4;

fn main() {
    let config = Config::new(env::args());
    let combo = Combo::new(config);

    println!("{}", combo.to_boxer_format());
}