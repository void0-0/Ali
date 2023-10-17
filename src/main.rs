mod combo;
mod config;

use std::env;
use combo::Combo;
use combo::ToFormat;
use config::Config;

// Add more readable format
// Add distance (step in and step out)
    // Make sure you hit a couple of times in between each steps

fn main() {
    let config = Config::new(env::args());
    let combo = Combo::new(&config);

    println!("{}", combo.to_format(&config.format));
}