use log::{error, info, trace};
use regex::Regex;
use std::env::{set_current_dir, temp_dir};
use std::{path::PathBuf, process::Command};
use std::fs::File;
use std::path::Path;
use compress_tools::Ownership;

pub fn extract(path: PathBuf) -> PathBuf {
    let tdir = Path::new(&temp_dir()).join(&path);
    compress_tools::uncompress_archive(File::open(path).unwrap(), &tdir, Ownership::Preserve).unwrap();
    tdir
}
