#![doc = include_str!("../README.md")]
pub mod functions;

use functions::*;
use log::{error, info};
use regex::Regex;
use rhai::Engine;
use std::{
    env::{
        args,
        temp_dir
    },
    path::Path,
    process::exit
};
use xdg_home::home_dir;


fn main() {
    colog::init();
    let mut parse = Engine::new();
    let arg: Vec<String> = args().collect();
    parse
        .register_fn("download", download)
        .register_fn("download_extract", download_extract)
        .register_fn("set_env", set_env)
        .register_fn("unset_env", unset_env)
        .register_fn("install", install)
        .register_fn("step", step)
        .register_fn("mkpackage", mkpackage)
        .register_fn("mkdir_chdir", mkdir_chdir)
        .register_fn("copy_remote", copy_remote)
        .register_fn("copy_local", copy_local)
        .register_fn("remote_step", remote_step);
    if arg.len() == 1 {
        info!("Type {} help for help", arg[0])
    }
    match arg[1].as_str() {
        "generate" => {
            let generated = "let name = ''\nlet desc = ''\nlet version = [0, 0, 0]\n# download";
            std::fs::write(&arg[2], generated).unwrap()
        }
        "docs" => {
            let docs = rhai_autodocs::export::options()
                .include_standard_packages(false)
                .export(&parse)
                .expect("failed to export documentation");
            let mdb = rhai_autodocs::generate::mdbook()
                .generate(&docs)
                .expect("failed to generate mdx for docusaurus");
            for (name, contents) in mdb {
                println!("{name}\n{contents}")
            }
        }
        "install" | "run" => {
            if arg[2..arg.len()].is_empty() {
                info!(
                    "Syntax: {} build [PACKAGE]\nType {} help for more information",
                    arg[0], arg[0]
                );
                exit(0);
            }
            for pkg in 2..=arg.len() - 1 {
                if Path::new(&home_dir().unwrap())
                    .join(format!(".mtos/pkgs/{}", arg[pkg]))
                    .exists()
                    && Path::new(&temp_dir())
                        .join(format!(".mtos/pkgs/{}", arg[pkg]))
                        .exists()
                {
                    info!(
                        "Use {} remove if you want to remove the package",
                        arg[0]
                    );
                    exit(1);
                }
                info!("Making package {}", arg[pkg]);
                parse
                    .eval_file::<()>(format!("{}.mt", arg[pkg]).into())
                    .unwrap_or_else(|e| {
                        error!("Script failed to run\n{e}");
                    });
            }
        }
        "remove" => {
            for remove in 2..arg.len() - 1 {
                match is_root() {
                    true => {
                        for dir in [
                            Path::new(
                                format!(".mtos/pkgs/{}", arg[remove]).as_str(),
                            ),
                            &*Path::new(&temp_dir()).join(&arg[remove]),
                        ] {
                            std::fs::remove_dir_all(dir).unwrap()
                        }
                    }
                    false => {
                        for dir in [
                            Path::new(home_dir().unwrap().as_path())
                                .join(format!(".mtos/pkgs/{}", arg[remove])),
                            Path::new(&temp_dir()).join(&arg[remove]),
                        ] {
                            std::fs::remove_dir_all(&dir).unwrap();
                            info!(
                                "Removed {} ({})",
                                arg[remove],
                                dir.display()
                            );
                        }
                    }
                }
                std::fs::remove_dir(
                    Path::new(&temp_dir()).join(
                        Regex::new(format!(r"{}*", arg[remove]).as_str())
                            .unwrap()
                            .as_str(),
                    ),
                )
                .unwrap();
                info!("Removed {}", arg[remove])
            }
        }
        "help" => {
            info!("Package manager");
            info!("Usage: {} OPTIONS [ARGUMENTS].\n", arg[0]);
            info!("Commands:\n");
            info!("generate [LOC]\tGenerate example configuration file.");
            info!("docs\t\tGenerate documentation for the build files. \
            \n\t\t\t└── Make sure you pipe it to tee or to redirect the output to Markdown file.");
            info!("install [PKG]\tInstall package.");
            info!("build [PKG]\tBuild package");
            info!("remove [PKG]\tRemove package.");
            info!("------------------------------\nFor listing packages, type ls ~/.mtos/pkgs or ls /mtos/pkgs if you're root.")
        }
        _ => {
            info!("Run {} help for help", arg[0])
        }
    }
}
