use log::{error, info};
use std::{
    env::temp_dir,
    path::Path,
    process::{exit, Command, ExitStatus},
    env::{remove_var, set_var}
};
use std::fs::{read_dir, write};

/// Installs the package.
/// Sets the INSTDIR environment variable for easy putting.
pub fn install(pkg_name: &str) {
    let path = Path::new(&temp_dir()).join("pkg");
    let mut paths: String = String::new();
    for i in read_dir(&paths).unwrap() {
        let p = i.unwrap();
        paths.push_str(p.path().to_str().unwrap())
    }
    Command::new("tar")
        .args([
            "-czvf",
            format!("{pkg_name}.pm.tar.gz").as_str(),
            path.to_str().unwrap(),
        ])
        .status()
        .unwrap();
    info!("DONE!")
}
/// Sets the variable
pub fn set_env(env: &str, var: &str) {
    set_var(env, var)
}
/// Unsets the variable
pub fn unset_env(env: &str) {
    remove_var(env)
}
/// The function that runs commands
pub fn step(name: &str, cmd: &str, args: &str) -> ExitStatus {
    info!("Running command {name}");
    let args: Vec<_> = args.split_whitespace().collect();
    Command::new(cmd).args(&args[0..args.len()]).status().unwrap()
}

/// Downloads and extracts the target file.
/// For only downloading or downloading binaries, use `download()` instead.
/// EXTRACT THE COMPRESSED FILE BEFOREHAND TO SEE IF ANY DIRECTORY CHANGE IS NEEDED.
pub fn download_extract(
    name: &str,
    file_name: &str,
    ext: &str,
    addr: &str,
    sha256: &str,
) {
    info!("Downloading {name} from {addr}");
    let p = Path::new(temp_dir().as_path()).join(format!("{file_name}{ext}"));
    fetch_data::download(&addr, &p).unwrap();
    let hash = fetch_data::hash_download(addr, &p).unwrap();
    if hash != sha256 {
        error!("FILE IS UNSAFE TO USE, STOPPING THE OPERATION NOW!!\nExpected {hash}, got {sha256}");
        exit(1);
    }
    std::env::set_current_dir(temp_dir()).unwrap();
    if ext.contains("tar") {
        Command::new("tar")
            .args(["-xvf", p.to_str().unwrap()])
            .status()
            .unwrap();
        std::env::set_current_dir(file_name).unwrap();
    } else if ext == "zip" {
        Command::new("unzip")
            .arg(p.to_str().unwrap())
            .status()
            .unwrap();
        std::env::set_current_dir(file_name).unwrap();
    } else {
        error!("Extension not supported\nGot {ext}");
        exit(1);
    }
}
/// Only downloads the given file.
pub fn download(
    name: &str,
    file_name: &str,
    ext: &str,
    addr: &str,
    sha256: &str,
) {
    info!("Downloading {name} from {addr}");
    let p = Path::new(temp_dir().as_path()).join(format!("{file_name}{ext}"));
    fetch_data::download(&addr, &p).unwrap();
    if fetch_data::hash_download(addr, &p).unwrap() != sha256 {
        error!("FILE IS UNSAFE TO USE, STOPPING THE OPERATION NOW!!");
        exit(1);
    }
}
