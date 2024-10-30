use clap::Parser;
use fetch_data::hash_download;
use log::info;
use package_manager::impls::Builder;
use std::{env::temp_dir, fs::read_to_string, path::Path};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long)]
    cfg: String,
}
fn main() {
    colog::init();
    let a = Args::parse();
    let f = read_to_string(Path::new(&a.cfg)).unwrap();
    let cfg: Builder = serde_yaml::from_str(&f).unwrap();
    for mut dl in cfg.dl {
        dl.sha256 = hash_download(
            dl.src,
            Path::new(temp_dir().as_path())
                .join(format!("{}.{}", dl.name, dl.ft)),
        )
        .unwrap();
        info!("Hash to include: {}", dl.sha256)
    }
}
