use crate::is_root;
use iter::once;
use log::{error, info, trace};
use regex::Regex;
use std::{
    env::{
        set_current_dir,
        remove_var,
        set_var,
        temp_dir
    },
    fs::{
        create_dir,
        read_dir,
        remove_dir_all,
        rename,
        DirBuilder
    },
    iter,
    os::unix::fs::symlink,
    path::Path,
    process::{
        exit,
        Command,
        ExitStatus
    }
};
use xdg_home::home_dir;
pub fn mkdir_chdir(dir: &str) {
    create_dir(dir).unwrap();
    set_current_dir(dir).unwrap()
}
/// Installs the package.
/// Sets the INSTDIR environment variable for easy putting.
pub fn install(pkg_name: &str) {
    match is_root() {
        true => {
            rename(
                Path::new(&temp_dir()).join("pkg").to_str().unwrap(),
                Path::new(&temp_dir()).join(pkg_name),
            )
            .unwrap();
            symlink(
                Path::new(&temp_dir()).join(pkg_name),
                format!("/mtos/pkgs/{pkg_name}"),
            )
            .unwrap();

            info!("Cleaning leftovers");
            for leftover in read_dir(temp_dir()).unwrap() {
                let left = leftover.unwrap();
                let x = Regex::new(
                    format!(r"{}-*", left.path().to_str().unwrap()).as_str(),
                )
                .unwrap();
                let path = Path::new(x.as_str());
                if path.exists() {
                    trace!("Cleaning {}", path.display());
                    remove_dir_all(path).unwrap()
                }
            }
        }
        false => {
            let path = Path::new(&home_dir().unwrap()).join(".mtos/pkgs");
            DirBuilder::new().recursive(true).create(&path).unwrap();

            let buf = Path::new(&temp_dir()).join("pkg");
            rename(
                buf.to_str().unwrap(),
                Path::new(&temp_dir()).join(pkg_name),
            )
            .unwrap();
            symlink(
                Path::new(&temp_dir()).join(pkg_name),
                format!("{}/{pkg_name}", path.to_str().unwrap()),
            )
            .unwrap();
            set_var(
                "PATH",
                format!("{}/{pkg_name}:$PATH", path.to_str().unwrap()),
            );
            info!("Cleaning leftovers");
            for leftover in read_dir(temp_dir()).unwrap() {
                let left = leftover.unwrap();
                let x = Regex::new(
                    format!(r"{}-*", left.path().to_str().unwrap()).as_str(),
                )
                .unwrap();
                let path = Path::new(x.as_str());
                if path.exists() {
                    trace!("Cleaning {}", path.display());
                    remove_dir_all(path).unwrap()
                }
            }
            if buf.exists() {
                remove_dir_all(buf).unwrap()
            }
        }
    }
    info!("DONE!")
}
/// Sets the variable
pub fn set_env(env: &str, var: &str) {
    set_var(env, var)
}
/// Packages the contents in the pkg directory.
pub fn mkpackage(name: &str) {
    let path = Path::new(&temp_dir()).join("pkg");
    let archive = format!("./{}.pm", name);
    Command::new("tar")
        .args(["-czvf", archive.as_str(), path.to_str().unwrap()])
        .status()
        .unwrap();
}
/// Unsets the variable
pub fn unset_env(env: &str) {
    remove_var(env)
}
/// The function that runs commands
pub fn step(name: &str, cmd: &str, args: &str) -> ExitStatus {
    info!("Running command {name}");
    let args: Vec<_> = args.split_whitespace().collect();
    Command::new(cmd)
        .args(&args[0..args.len()])
        .status()
        .unwrap()
}

/// Downloads and extracts the target file.
/// For only downloading or downloading binaries, use `download()` instead.
/// EXTRACT THE COMPRESSED FILE BEFOREHAND TO SEE IF ANY DIRECTORY CHANGE IS NEEDED.
/// - if so, use the [`download`] function instead in combination with [`step`].
pub fn download_extract(
    name: &str,
    file_name: &str,
    ext: &str,
    addr: &str,
    sha256: &str,
) {
    info!("Downloading {name} from {addr}");
    let p = Path::new(temp_dir().as_path()).join(format!("{file_name}{ext}"));
    fetch_data::download(addr, &p).unwrap();
    let hash = fetch_data::hash_download(addr, &p).unwrap();
    if hash != sha256 {
        error!("FILE IS UNSAFE TO USE, STOPPING THE OPERATION NOW!!\nExpected {hash}, got {sha256}");
        exit(1);
    }
    set_current_dir(temp_dir()).unwrap();
    if ext.contains("tar") || ext.contains(".tgz") {
        Command::new("tar")
            .args(["-xvf", p.to_str().unwrap()])
            .status()
            .unwrap();
        set_current_dir(file_name).unwrap();
    } else if ext == "zip" {
        Command::new("unzip")
            .arg(p.to_str().unwrap())
            .status()
            .unwrap();
        set_current_dir(file_name).unwrap();
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
    fetch_data::download(addr, &p).unwrap();
    if fetch_data::hash_download(addr, &p).unwrap() != sha256 {
        error!("FILE IS UNSAFE TO USE, STOPPING THE OPERATION NOW!!");
        exit(1);
    }
}
pub fn copy_remote(host: &str, source: &str, dest: &str) {
    Command::new("scp")
        .args([source, format!("{host}:{dest}").as_str()])
        .status()
        .unwrap();
}
pub fn copy_local(source: &str, dest: &str) {
    Command::new("cp").args([source, dest]).status().unwrap();
}
pub fn remote_step(host: &str, cmd: &str, args: &str) -> ExitStatus {
    let after_split: Vec<_> = args.split_whitespace().collect();
    Command::new("ssh")
        .args(
            once(host)
                .chain(once(cmd))
                .chain(after_split.iter().copied()),
        )
        .status()
        .unwrap()
}
