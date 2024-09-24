#![warn(clippy::all, clippy::pedantic)]
#![cfg(target_os = "linux")]
use clap::Parser;
use impls::Builder;
use traits::Building;
mod impls;
mod macros;
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
    if let Some(pm) = arge.install {
        for i in pm {
            {
                let pkg = i.as_str();
                let b = crate::impls::Builder::default();
                b.prep(pkg).unwrap();
                b.build(pkg).unwrap();
                b.install(pkg).unwrap();
            }
        }
    }
    if let Some(pm) = arge.remove {
        for i in pm {
            {
                let pkg = i.as_str();
                let b = crate::impls::Builder::default();
                b.remove(pkg).unwrap();
            }
        }
    }
    if let Some(pm) = arge.query {
        for i in pm {
            {
                let pkg = i.as_str();
                let b = crate::impls::Builder::default();
                b.query(pkg).unwrap()
            }
        }
    }
    if let Some(cfg) = arge.create {
        Builder::default().write(cfg.as_str()).unwrap()
    }
}
