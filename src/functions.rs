use iter::once;
use log::{error, info, trace};
use regex::Regex;
use std::os::unix::prelude::ExitStatusExt;
use std::{
    env::{
        remove_var,
        set_current_dir,
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
use std::env::var;
use xdg_home::home_dir;
pub fn is_root() -> bool {
    let user = var("USER");
    Command::new("id")
        .args(["-u", user.unwrap().as_str()])
        .status()
        .unwrap()
        .code()
        .unwrap()
        == 1000
}
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
/// Copies a file from the local system to a remote system using SCP.
/// 
/// # Arguments
/// 
/// * `host` - The remote host to copy the file to.
/// * `source` - The source file path on the local system.
/// * `dest` - The destination file path on the remote system.
/// 
/// # Example
/// 
/// ```
/// copy_remote("user@example.com", "/local/path/file.txt", "/remote/path/file.txt");
/// ```
pub fn copy_remote(host: &str, source: &str, dest: &str) {
    Command::new("scp")
        .args([source, format!("{host}:{dest}").as_str()])
        .status()
        .unwrap();
}

/// Copies a file from one location to another on the local system.
/// 
/// # Arguments
/// 
/// * `source` - The source file path on the local system.
/// * `dest` - The destination file path on the local system.
/// 
/// # Example
/// 
/// ```
/// copy_local("/local/path/file.txt", "/new/path/file.txt");
/// ```
pub fn copy_local(source: &str, dest: &str) {
    Command::new("cp").args([source, dest]).status().unwrap();
}

/// Executes a command on a remote system over SSH.
/// 
/// # Arguments
/// 
/// * `host` - The remote host to connect to.
/// * `cmd` - The command to run on the remote host.
/// * `args` - A string of arguments to pass to the command.
/// 
/// # Returns
/// 
/// Returns the exit status of the remote command.
/// 
/// # Example
/// 
/// ```
/// let status = remote_step("user@example.com", "ls", "-l /home");
/// ```
pub fn remote_step(host: &str, cmd: &str, args: &str) -> ExitStatus {
    info!("Running remote step on {host} with {cmd} {args}");
    let args: Vec<_> = args.split_whitespace().collect();
    Command::new("ssh")
        .args(
            once(host)
                .chain(once(cmd))
                .chain(args.iter().copied()),
        )
        .status()
        .unwrap()
}
/// Runs a remote command on a specified host and returns the termination signal of the command.
/// 
/// # Arguments 
/// 
/// * `host`: The hostname and IP address of the remote machine.
/// * `cmd`: The command to execute on the remote machine.
/// * `args`: Arguments to pass to the command.
/// 
/// returns: i32 - The signal number that caused the command to terminate 
/// 
/// # Examples 
/// 
/// ```
/// let signal = remote_step_unary("user@example.com", "ls", "-la");
/// println!("Terminated with signal: {}", signal);
pub fn remote_step_unary(host: &str, cmd: &str, args: &str) -> i32 {
    info!("Running remote step on {host} with {cmd} {args}");
    let args: Vec<_> = args.split_whitespace().collect();
    Command::new("ssh")
        .args(
            once(host)
                .chain(once(cmd))
                .chain(args.iter().copied()),
        )
        .status()
        .unwrap().signal().unwrap()
}