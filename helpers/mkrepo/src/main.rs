use clap::Parser;
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use std::process::exit;
use std::{
    fs::{read_dir, rename, write, DirBuilder},
    io,
    path::Path,
};

// Structure to represent the repository configuration.
#[derive(Serialize, Deserialize, Default)]
struct RepoConfig {
    name: String,
    pkgs: Vec<String>,
}

// Structure to parse command-line arguments.
#[derive(Parser)]
#[clap(version, about, long_about = None)]
struct Cli {
    #[clap(long = "create")]
    /// creates an repo
    repo: String,
}

fn main() -> io::Result<()> {
    // Initialize logging.
    colog::init();

    // Create a default repository configuration.
    let mut name = RepoConfig::default();

    // Parse command-line arguments.
    let arge = Cli::parse();

    // Set the repository name from the command-line arguments.
    name.name = arge.repo.clone();

    // Log that the repository is being populated.
    info!(target: "setup", "Populating repo {}", name.name);

    // Check if the "/mtos" directory exists.
    if !Path::new("/mtos").exists() {
        // Log a warning if the directory doesn't exist and create it.
        warn!(target: "setup", "No such repo {}, creating it right now", name.name);
        DirBuilder::new().recursive(true).create(&name.name)?;
    }

    // Iterate over the entries in the specified repository directory.
    for ent in read_dir(&arge.repo)? {
        let entry = ent?;
        let path = entry.path();

        // Check if the file has the "mtos.yml" extension.
        match path.extension().unwrap() == "mtos.yml" {
            true => {
                // Log that the file is being processed.
                info!(target: "populating", "Processing file {}", path.display());

                // Move the file to the "/mtos" directory.
                rename(&path, Path::new("/mtos").join(&path))?;

                // Add the file path to the list of packages.
                name.pkgs.push(path.to_str().unwrap().to_string());
            }
            false => {
                // Log an error if the file doesn't have the correct extension and exit.
                error!("File does not have a mtos.yml extension, exiting");
                exit(1)
            }
        };
    }

    // Log that the repository is being created.
    info!(target: "creation", "Creating repo {}", name.name);

    // Write the repository configuration to the "config.yml" file.
    write("/mtos/config.yml", serde_yaml::to_string(&name).unwrap())?;

    // Log that the process is complete.
    info!("DONE");

    Ok(())
}
