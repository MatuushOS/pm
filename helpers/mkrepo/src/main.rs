use std::{
    env::args,
    fs::{read_dir, rename, write, DirBuilder},
    io,
    path::Path,
};

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Default)]
struct RepoConfig {
    name: String,
    pkgs: Vec<String>,
}

fn main() -> io::Result<()> {
    let mut name = RepoConfig::default();
    let arge: String = args().collect();
    name.name = arge;
    if !Path::new("/mtos").exists() {
        DirBuilder::new()
            .recursive(true)
            .create(&name.name)
            .unwrap();
    }
    for entry in read_dir(".")? {
        let entry = entry?;
        let path = entry.path();
        match Path::new(&path).extension().unwrap() == "yml" {
            true => {
                rename(&path, Path::new("/mtos").join(&path))?;
                name.pkgs.push(path.to_str().unwrap().to_string());
            }
            false => break,
        };
    }
    write("/mtos/config.yml", serde_yaml::to_string(&name).unwrap()).unwrap();
    Ok(())
}
