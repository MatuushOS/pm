#![warn(clippy::all, clippy::pedantic, clippy::perf)]

#[cfg(feature = "gui")]
use clap::CommandFactory;
use clap::Parser;
#[cfg(any(feature = "gui", feature = "nogui"))]
use impls::Builder;
use log::error;
#[cfg(any(feature = "gui", feature = "nogui"))]
use traits::{Building, DependencyResolution, Filling};
pub mod impls;
#[derive(Parser)]
#[clap(name = "pm", author = "Matus Mastena <Shadiness9530@proton.me>", about = "Package manager", long_about = None)]
struct Cli {
    #[arg(short, long)]
    /// Installs a package
    install: Option<String>,
    #[arg(short, long)]
    /// Removes a package
    remove: Option<String>,
    #[arg(short, long)]
    /// Queries a package
    query: Option<String>,
    #[arg(short, long)]
    /// Creates a new configuration file
    create: Option<String>,
}
macro_rules! infill {
    ($var:expr) => {
        #[cfg(target_os = "windows")]
        let c = std::fs::read_to_string(std::path::Path::new(&$var)).unwrap();
        if c.contains("prepare") || c.contains("build") {
            error!("prepare or build failed");
        }
    };
}
fn main() {
    colog::init();
    #[cfg(feature = "nogui")]
    let arge = Cli::parse();
    #[cfg(feature = "gui")]
    let gui = Cli::command();
    #[cfg(feature = "gui")]
    claui::run(gui, move |matches| {
        if matches.get_flag("remove") {
            for p in arge.remove.clone().unwrap().chars() {
                let f = std::fs::read_to_string(p.to_string()).unwrap();
                let mut b = Builder::default();
                b.fill(f.as_str().into()).unwrap();
                b.remove(&f).unwrap();
            }
        } else if matches.get_flag("install") {
            for i in arge.install.clone().unwrap().chars() {
                let f = std::fs::read_to_string(i.to_string()).unwrap();
                let mut b = Builder::default();
                b.fill(f.as_str().parse().unwrap()).unwrap();
                b.resolve().unwrap();
            }
        } else if matches.get_flag("query") {
            for q in arge.query.clone().unwrap().iter() {
                let f = std::fs::read_to_string(q).unwrap();
                let mut b = Builder::default();
                b.fill(f.as_str().into()).unwrap();
                b.remove().unwrap();
            }
        } else if matches.get_flag("create") {
            Builder::default()
                .write(arge.create.clone().unwrap().as_str())
                .unwrap();
        }
    })
    .unwrap();
    #[cfg(feature = "nogui")]
    if arge.remove.clone().is_some() {
        let mut b = Builder::default();
        infill!(&arge.remove.clone().unwrap());
        b.fill(arge.remove.as_ref().unwrap().into()).unwrap();
        b.remove().unwrap();
    } else if arge.install.is_some() {
        infill!(&arge.install.clone().unwrap());
        let mut b = Builder::default();
        b.fill(arge.install.unwrap().as_str().into()).unwrap();
        b.resolve().unwrap();
    } else if arge.query.is_some() {
        let mut b = Builder::default();
        b.fill(arge.query.unwrap().as_str().into()).unwrap();
        b.query().unwrap();
    } else if arge.create.is_some() {
        Builder::default()
            .write(arge.create.unwrap().as_str())
            .unwrap();
    }
}
