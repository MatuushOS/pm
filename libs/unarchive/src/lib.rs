use std::{path::PathBuf, process::Command};

use log::{error, trace};

pub fn extract(f: PathBuf) {
    match f.as_path().extension() {
        Some(ext) => match ext.to_ascii_lowercase().into_string().unwrap().as_str() {
            "tar.gz" => {
                match Command::new("tar")
                    .args(["-xvf", f.as_path().to_str().unwrap(), "-C", "src/"])
                    .output()
                {
                    Ok(ok) => trace!("extraction complete, {ok:#?}"),
                    Err(e) => error!("extraction failed, {e:#?}"),
                }
            }
            "tar.bz2" => {
                match Command::new("tar")
                    .args(["-xvf", f.as_path().to_str().unwrap(), "-C", "src/"])
                    .output()
                {
                    Ok(ok) => trace!("extraction complete, {ok:#?}"),
                    Err(e) => error!("extraction failed, {e:#?}"),
                }
            }
            ".zip" => match Command::new("unzip")
                .args([f.as_path().to_str().unwrap(), "-d", "src/"])
                .output()
            {
                Ok(ok) => trace!("extraction complete, {ok:#?}"),
                Err(e) => error!("extraction failed, {e:#?}"),
            },
            _ => error!("Unknown extension"),
        },
        None => error!("No extension provided"),
    }
}
