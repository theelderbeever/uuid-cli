use std::fmt::Display;

use clap::{Parser, ValueEnum};
use uuid::Uuid;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, arg_required_else_help = false)]
pub struct Cli {
    #[arg(default_value_t = Version::V4)]
    uuid: Version,
    #[arg(short, default_value_t = 1, help = "Number of UUIDs to generate.")]
    n: u64,
}

#[derive(ValueEnum, Clone, Debug)]
enum Version {
    V1,
    V3,
    V4,
    V5,
    V6,
    V7,
    V8,
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::V1 => f.write_str("v1"),
            Self::V3 => f.write_str("v3"),
            Self::V4 => f.write_str("v4"),
            Self::V5 => f.write_str("v5"),
            Self::V6 => f.write_str("v6"),
            Self::V7 => f.write_str("v7"),
            Self::V8 => f.write_str("v8"),
        }
    }
}

fn main() {
    let args = Cli::parse();

    for _ in 0..args.n {
        match args.uuid {
            Version::V1 => todo!(),
            Version::V3 => todo!(),
            Version::V4 => println!("{}", Uuid::new_v4()),
            Version::V5 => todo!(),
            Version::V6 => todo!(),
            Version::V7 => todo!(),
            Version::V8 => todo!(),
        }
    }
}
