#![warn(clippy::all, clippy::pedantic)]
#![cfg(target_os = "linux")]
use std::fs::read_to_string;

use clap::Parser;
use impls::Builder;
use traits::{Building, DependencyResolution};
pub mod impls;
pub mod macros;
#[derive(Parser)]
#[clap(name = "pm", about = "Package manager", long_about = None)]
struct Cli {
    #[arg(short, long)]
    install: Option<Vec<String>>,
    #[arg(short, long)]
    remove: Option<Vec<String>>,
    #[arg(short, long)]
    query: Option<Vec<String>>,
    #[arg(short, long)]
    create: Option<String>,
}

fn main() {
    let arge = Cli::parse();
    for p in arge.remove.unwrap() {
        let f = std::fs::read_to_string(p).unwrap();
        let cfg: Builder = serde_yaml::from_str(&f).unwrap();
        
    }
}
