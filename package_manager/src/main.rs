#![warn(clippy::all, clippy::pedantic, clippy::perf)]
#![cfg(target_os = "linux")]
use clap::Parser;
use impls::Builder;
use traits::{Building, DependencyResolution, Filling};
pub mod impls;
pub mod macros;
#[derive(Parser)]
#[clap(name = "pm", about = "Package manager", long_about = None)]
struct Cli {
    #[arg(short, long)]
    /// Installs a package
    install: Option<Vec<String>>,
    #[arg(short, long)]
    /// Removes a package
    remove: Option<Vec<String>>,
    #[arg(short, long)]
    /// Queries a package
    query: Option<Vec<String>>,
    #[arg(short, long)]
    /// Creates a new configuration file
    create: Option<String>,
}

fn main() {
    let arge = Cli::parse();
    if arge.remove.is_some() {
        for p in arge.remove.unwrap() {
            let f = std::fs::read_to_string(p).unwrap();
            let mut b = Builder::default();
            b.fill(f.as_str()).unwrap();
            b.remove(&f).unwrap();
        }
    } else if arge.install.is_some() {
        for i in arge.install.unwrap() {
            let f = std::fs::read_to_string(i).unwrap();
            let mut b = Builder::default();
            b.fill(f.as_str()).unwrap();
            b.resolve().unwrap();
        }
    } else if arge.query.is_some() {
        for q in arge.query.unwrap() {
            let f = std::fs::read_to_string(q).unwrap();
            let mut b = Builder::default();
            b.fill(f.as_str()).unwrap();
            b.remove(&f).unwrap();
        }
    } else if arge.create.is_some() {
        Builder::default()
            .write(arge.create.unwrap().as_str())
            .unwrap();
    }
}
