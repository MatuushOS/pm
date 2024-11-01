#![doc = include_str!("../../README.md")]
#![warn(clippy::all, clippy::pedantic, clippy::perf)]

#[cfg(feature = "gui")]
use clap::CommandFactory;
use clap::Parser;
use impls::Builder;
use std::io;
use traits::{DependencyResolution, Filling};

pub mod impls;
macro_rules! install {
    ($pkg:expr) => {
        #[cfg(target_os = "windows")]
        infill!(&$pkg);
        let mut b = Builder::default();
        b.fill($pkg.into()).unwrap();
        b.resolve().unwrap();
    };
}
/// CLI arguments struct.
#[derive(Parser)]
#[clap(name = "pm", author = "Matus Mastena <Shadiness9530@proton.me>", about = "Package manager", long_about = None)]
struct Cli {
    /// Installs a package.
    #[arg(short, long)]
    install: Option<String>,
    /// Removes a package.
    #[arg(short, long)]
    remove: Option<String>,
    /// Queries a package.
    #[arg(short, long)]
    query: Option<String>,
    /// Creates a new configuration file.
    #[arg(short, long)]
    create: Option<String>,
}
/// Macro for infilling on Windows.
#[cfg(target_os = "windows")]
macro_rules! infill {
    ($var:expr) => {
        #[cfg(target_os = "windows")]
        let c = std::fs::read_to_string(std::path::Path::new(&$var)).unwrap();
        if c.contains("prepare") || c.contains("build") {
            error!("prepare or build failed");
            std::process::exit(1)
        }
    };
}
/// Macro for GUI.
#[cfg(feature = "gui")]
macro_rules! gui {
    ($arge:expr) => {
        let arge = $arge;
        #[cfg(feature = "gui")]
        let gui = Cli::command();
        #[cfg(feature = "gui")]
        claui::run(gui, move |matches| {
            match (
                matches.get_flag("remove"),
                matches.get_flag("install"),
                matches.get_flag("query"),
                matches.get_flag("create"),
            ) {
                (true, _, _, _) => {
                    for p in arge.remove.clone().unwrap().chars() {
                        let f = std::fs::read_to_string(p.to_string()).unwrap();
                        let mut b = Builder::default();
                        b.fill(f.as_str().into());
                        b.remove();
                    }
                }
                (_, true, _, _) => {
                    for i in arge.install.clone().unwrap().chars() {
                        macro_rules! install {
                            ($pkg:expr) => {
                                #[cfg(target_os = "windows")]
                                infill!(&$pkg);
                                let mut b = Builder::default();
                                b.fill($pkg.into()).unwrap();
                                b.resolve().unwrap();
                            };
                        }               let f = std::fs::read_to_string(i.to_string()).unwrap();
                        let mut b = Builder::default();
                        b.fill(f.as_str().parse().unwrap()).unwrap();
                        b.resolve().unwrap();
                    }
                }
                (_, _, true, _) => {
                    if let Some(q) = arge.query.clone() {
                        let f = std::fs::read_to_string(q).unwrap();
                        let mut b = Builder::default();
                        b.fill(f.as_str().into()).unwrap();
                        b.remove().unwrap();
                    }
                }
                (_, _, _, true) => {
                    Builder::write(arge.create.clone().unwrap().as_str());
                }
                _ => (),
            }
        });
    };
}
/// Main function when GUI feature is enabled.
#[cfg(feature = "gui")]
fn main() {
    let arge = Cli::parse();
    #[cfg(feature = "gui")]
    gui!(arge);
}
/// Main function when GUI feature is disabled.
#[cfg(not(feature = "gui"))]
fn main() -> io::Result<()> {
    colog::init();
    let arge = Cli::parse();
    #[cfg(not(feature = "gui"))]
    Ok(match (arge.remove, arge.install, arge.query, arge.create) {
        (Some(pkg), _, _, _) => {
            install!(pkg);
        }
        (_, Some(pkg), _, _) => {
            install!(pkg);
        }
        (_, _, Some(pkg), _) => {
            install!(pkg);
        }
        (_, _, _, Some(path)) => {
            Builder::write(path.as_str()).unwrap();
        }
        _ => (),
    })
}
