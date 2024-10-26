use clap::Parser;
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use std::process::exit;
use std::{
    fs::{read_dir, rename, write, DirBuilder},
    io,
    path::Path,
};
#[derive(Serialize, Deserialize, Default)]
struct RepoConfig {
    name: String,
    pkgs: Vec<String>,
}
#[derive(Parser)]
#[clap(version, about, long_about = None)]
struct Cli {
    #[clap(long = "create")]
    /// creates an repo
    repo: String,
}
fn main() -> io::Result<()> {
    colog::init();
    let mut name = RepoConfig::default();
    let arge = Cli::parse();
    name.name = arge.repo.clone();
    info!(target: "setup", "Populating repo {}", name.name);
    if !Path::new("/mtos").exists() {
        warn!(target: "setup", "No such repo {}, creating it right now", name.name);
        DirBuilder::new().recursive(true).create(&name.name)?;
    }
    for ent in read_dir(&arge.repo)? {
        let entry = ent?;
        let path = entry.path();
        match path.extension().unwrap() == "mtos.yml" {
            true => {
                info!(target: "populating", "Processing file {}", path.display());
                rename(&path, Path::new("/mtos").join(&path))?;
                name.pkgs.push(path.to_str().unwrap().to_string());
            }
            false => {
                error!("File does not have a mtos.yml extension, exiting");
                exit(1)
            }
        };
    }
    info!(target: "creation", "Creating repo {}", name.name);
    write("/mtos/config.yml", serde_yaml::to_string(&name).unwrap())?;
    info!("DONE");
    Ok(())
}
