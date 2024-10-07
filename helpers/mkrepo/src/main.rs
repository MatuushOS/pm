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
    for i in read_dir(".").unwrap() {
        let p = i.unwrap();
        match Path::new(p.path().as_path()).extension().unwrap() {
            p => {
                match p == "yml" {
                    true => {
                        rename(p, Path::new("/mtos").join(p))?;
                        name.pkgs.push(p.to_str().unwrap().to_string());
                    }
                    false => break,
                }
            }
            _ => break,
        };
    }
    write("/mtos/config.yml", serde_yaml::to_string(&name).unwrap()).unwrap();
    Ok(())
}
