#[cfg(test)]
mod tests {
    use std::env::home_dir;
    use std::path::Path;
    use std::{fs::read_dir, process::Command};

    #[test]
    fn build_all() {
        for f in read_dir("examples").unwrap() {
            let script = f.unwrap().path();
            let x = Path::new(script.as_path()).to_str().unwrap();
            let x1 = x.trim_end_matches(".mt");
            let cmd = Command::new("target/debug/pm")
                .args(["install", x1])
                .status();
            assert_eq!(cmd.unwrap().success(), true);
        }
    }
}
