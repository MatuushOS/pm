#[cfg(test)]
mod tests {
    use log::info;
    use std::env::current_dir;
    use std::{fs::read_dir, path::Path, process::Command};

    #[test]
    fn build_all() {
        colog::init();
        for f in read_dir(Path::new(&current_dir().unwrap()).join("examples"))
            .unwrap()
        {
            for scr in read_dir(Path::new(&f.unwrap().path())).unwrap() {
                let script = scr.unwrap().path();
                let x = Path::new(script.as_path()).to_str().unwrap();
                info!("Opening {x}");
                let x1 = x.trim_end_matches(".mt");
                let cmd = Command::new("target/debug/pm")
                    .args(["install", x1])
                    .status();
                assert!(cmd.unwrap().success())
            }
        }
    }
}
