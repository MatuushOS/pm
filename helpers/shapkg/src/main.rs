use std::{
    env::{args, temp_dir},
    fs::read_to_string,
    path::Path,
};

use fetch_data::hash_download;
use package_manager::impls::Builder;
fn main() {
    let a: Vec<String> = args().collect();
    for i in a {
        let f = read_to_string(i).unwrap();
        let cfg: Builder = serde_yaml::from_str(&f).unwrap();
        for mut dl in cfg.dl {
            dl.sha256 = hash_download(dl.src, Path::new(temp_dir().as_path())).unwrap()
        }
    }
}
