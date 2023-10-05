mod stance;
pub mod side;

use std::fs;
use std::env::Args;
pub use stance::Stance;
use toml;
use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub allow_same_side_punch: bool,
    pub allow_body_punch: bool,
    pub allow_head_punch: bool,
    pub allow_steps: bool,
    pub separator: String,
    pub stance: Stance,
    pub length: i32
}

impl Config {
    pub fn new(raw_args: Args) -> Config {
        let config_file_path: String = match raw_args.skip(1).next().and_then(|first_arg| first_arg.parse::<String>().ok()) {
            Some(content_file_path) => content_file_path,
            None => {
                println!("Incorrect arguments passed");
                println!("Loading default Config...");
                return Config::default();
            }
        };

        let config_file_content = match fs::read_to_string(config_file_path) {
            Ok(content) => content,
            Err(_) => {
                println!("File not found");
                println!("Loading default Config...");
                return Config::default();
            }
        };

        let mut config: Config =  match toml::from_str(config_file_content.as_str()) {
            Ok(config) => config,
            Err(_) => {
                println!("Unable to load config");
                println!("Loading default Config...");
                return Config::default();
            }
        };

        if !vec![config.allow_body_punch, config.allow_head_punch].contains(&true) {
            println!("Targets are invalid");
            println!("Overwriting target of given config...");
            config.allow_head_punch = true;
            config.allow_body_punch = true;
        }

        config
    }

    pub fn default() -> Config {
        Config {
            allow_same_side_punch: false,
            allow_body_punch: true,
            allow_head_punch: true,
            allow_steps: false,
            separator: ", ".to_string(),
            stance: Stance::Orthodox,
            length: 4,
        }
    }
}
