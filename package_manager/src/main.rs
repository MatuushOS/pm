#![doc = include_str!("../../README.md")]
#![warn(clippy::all, clippy::pedantic, clippy::perf)]

#[cfg(feature = "gui")]
use clap::CommandFactory;
use clap::Parser;
use impls::Builder;
use log::error;
use std::io;
use traits::{Building, DependencyResolution, Filling};

pub mod impls;
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
            if matches.get_flag("remove") {
                for p in arge.remove.clone().unwrap().chars() {
                    let f = std::fs::read_to_string(p.to_string()).unwrap();
                    let mut b = Builder::default();
                    b.fill(f.as_str().into());
                    b.remove();
                }
            } else if matches.get_flag("install") {
                for i in arge.install.clone().unwrap().chars() {
                    let f = std::fs::read_to_string(i.to_string()).unwrap();
                    let mut b = Builder::default();
                    b.fill(f.as_str().parse().unwrap()).unwrap();
                    b.resolve().unwrap();
                }
            } else if matches.get_flag("query") {
                if let Some(q) = arge.query.clone() {
                    let f = std::fs::read_to_string(q).unwrap();
                    let mut b = Builder::default();
                    b.fill(f.as_str().into()).unwrap();
                    b.remove().unwrap();
                }
            } else if matches.get_flag("create") {
                Builder::write(arge.create.clone().unwrap().as_str());
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
    Ok(if arge.remove.clone().is_some() {
        let mut b = Builder::default();
        #[cfg(target_os = "windows")]
        infill!(&arge.remove.clone().unwrap());
        b.fill(arge.remove.as_ref().unwrap().into()).unwrap();
        b.remove().unwrap();
    } else if arge.install.is_some() {
        #[cfg(target_os = "windows")]
        infill!(&arge.install.clone().unwrap());
        let mut b = Builder::default();
        b.fill(arge.install.unwrap().as_str().into()).unwrap();
        b.resolve().unwrap();
    } else if arge.query.is_some() {
        let mut b = Builder::default();
        b.fill(arge.query.unwrap().as_str().into()).unwrap();
        b.query().unwrap();
    } else if arge.create.is_some() {
        crate::impls::Builder::write(arge.create.unwrap().as_str()).unwrap();
    })
}