mod functions;

use log::{error, info};
use rhai::Engine;
use rhai_autodocs::export::SectionFormat;
use std::{
    env::args,
    process::exit
};

fn main() {
    colog::init();
    let mut parse = Engine::new();
    let arg: Vec<String> = args().collect();
    parse
        .register_fn("download", functions::download)
        .register_fn("download_extract", functions::download_extract)
        .register_fn("set_env", functions::set_env)
        .register_fn("unset_env", functions::unset_env)
        .register_fn("install", functions::install)
        .register_fn("step", functions::step);
    match arg[1].as_str() {
        "generate" => {
            let generated = "let name = ''\nlet desc = ''\nlet version = [0, 0, 0]\n# download";
            std::fs::write(&arg[2], generated).unwrap()
        }
        "docs" => {
            let docs = rhai_autodocs::export::options()
                .format_sections_with(SectionFormat::Tabs)
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
        "build" => {
            if arg[2..arg.len()].is_empty() {
                info!(
                    "Syntax: {} build [PACKAGE]\nType {} help for more information",
                    arg[0], arg[0]
                );
                exit(0);
            }
            for pkg in 2..=arg.len() - 1 {
                info!("Making package {}", arg[pkg]);
                parse
                    .eval_file::<()>(format!("{}.mt", arg[pkg]).into())
                    .unwrap_or_else(|e|  {
                       error!("Script failed to run\n{e}");
                    });
            }
        }
        "help" | _ => {
            info!("Package manager");
            info!("Usage: {} OPTIONS [ARGUMENTS]\n", arg[0]);
            info!("Commands:\n");
            info!("generate [LOC]\tGenerate example configuration file");
            info!("docs\t\tGenerate documentation. Make sure you pipe it to tee or to redirect the output to Markdown file");
            info!("build [PKG]\t\tBuild file")
        }
    }
}
