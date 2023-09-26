mod handedness;

use std::fs;
use std::env::Args;
use handedness::Handedness;
use toml;
use serde_derive::Deserialize;

const DEFAULT_COMBO_LENGTH: i32 = 4;

#[derive(Deserialize)]
pub struct Config {
    pub allow_same_side_punch: bool,
    pub allow_body_punch: bool,
    pub allow_head_punch: bool,
    pub allow_steps: bool,
    pub separator: String,
    pub handedness: Handedness,
    pub length: i32
}

impl Config {
    pub fn new(mut raw_args: Args) -> Config {
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

        return match toml::from_str(config_file_content.as_str()) {
            Ok(config) => config,
            Err(_) => {
                println!("Unable to load config");
                println!("Loading default Config...");
                return Config::default();
            }
        };
    }

    pub fn default() -> Config {
        Config {
            allow_same_side_punch: true,
            allow_body_punch: true,
            allow_head_punch: true,
            allow_steps: false,
            separator: ", ".to_string(),
            handedness: Handedness::RightHanded,
            length: DEFAULT_COMBO_LENGTH,
        }
    }
}