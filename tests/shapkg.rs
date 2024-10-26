#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use std::process::Command;

    #[test]
    fn try_hash() {
        let a = assert!(Command::new(
            Path::new("target")
                .join("debug")
                .join("shapkg")
                .to_str()
                .unwrap()
        ).args(["tests/hello-unix.yml"]));
        a.status().unwrap().success();
    }
}
