use clap::{ValueEnum, Parser};
use serde::Deserialize;

/// Battle simulation
#[derive(Debug, Deserialize, Parser)]
pub struct Args {
    /// Fighter type, either pokemon or rps
    #[clap(value_enum, short='t', long, default_value_t = default_fighter_type())]
    #[serde(default = "default_fighter_type")]
    pub fighter_type: FighterType,

    /// Image width
    #[clap(short='x', long, default_value_t = default_size(), value_parser = validate_size)]
    #[serde(default = "default_size")]
    pub width: usize,

    /// Image height
    #[clap(short='y', long, default_value_t = default_size(), value_parser = validate_size)]
    #[serde(default = "default_size")]
    pub height: usize,

    /// When fighting, select random neighbour instead of the weakest one
    #[clap(short = 'r', long)]
    #[serde(default)]
    pub random: bool,

    /// Measure frame rate and print it to stdout
    #[clap(short = 'f', long)]
    #[serde(default)]
    pub framerate: bool,

    /// Let fighters fight their own kind
    #[clap(short = 'o', long)]
    #[serde(default)]
    pub fightown: bool,
}

fn default_fighter_type() -> FighterType {
    FighterType::StreetFighter
}

fn default_size() -> usize {
    512
}

#[derive(ValueEnum, Clone, Debug, Deserialize)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub enum FighterType {
    Pokemon,
    RockPaperScissors,
    StreetFighter,
}

fn validate_size(arg: &str) -> Result<usize, String> {
    if let Ok(size) = arg.parse::<usize>() {
        // wgpu won't allow more than 8192 pixels
        if !(32..8193).contains(&size) {
            return Err("image size should be between 32 and 8192".to_string());
        } else {
            return Ok(size);
        }
    }

    Err("Size is not a number".to_string())
}
