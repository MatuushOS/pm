#[cfg(test)]
mod tests {
    use std::{
        fs::read_dir,
        process::Command
    };
    use std::path::Path;
    
    #[test]
    fn build_all() {
        for f in read_dir("examples").unwrap() {
            let script = f.unwrap().path();
            let x = Path::new(script.as_path()).to_str().unwrap();
            let cmd = Command::new("target/debug/pm")
                .args(["install", x.trim_end_matches(".mt")])
                .status();
            assert_eq!(cmd.unwrap().success(), true)
        }
    }
}
