use log::{error, trace};
use regex::Regex;
use std::env::set_current_dir;
use std::{path::PathBuf, process::Command};

pub fn extract(f: PathBuf) {
    let re = Regex::new(r"\.tar(\.gz|\.bz2)?$").unwrap();

    match f.as_path().extension() {
        Some(ext) => {
            let ext_str = ext.to_ascii_lowercase().into_string().unwrap();
            if re.is_match(&ext_str) {
                match Command::new("tar")
                    .args(["-xvf", f.as_path().to_str().unwrap()])
                    .output()
                {
                    Ok(ok) => {
                        trace!("extraction complete, {ok:#?}");
                        set_current_dir(f.as_path().to_str().unwrap().replace(
                            f.as_path().extension().unwrap().to_str().unwrap(),
                            " ",
                        ))
                        .unwrap();
                    }
                    Err(e) => error!("extraction failed, {e:#?}"),
                }
            } else if ext_str == "zip" {
                match Command::new("unzip")
                    .args([f.as_path().to_str().unwrap()])
                    .output()
                {
                    Ok(ok) => trace!("extraction complete, {ok:#?}"),
                    Err(e) => error!("extraction failed, {e:#?}"),
                }
            } else {
                error!("Unknown extension")
            }
        }
        None => error!("No extension provided"),
    }
}
