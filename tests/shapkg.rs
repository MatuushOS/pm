#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::process::Command;
    #[test]
    #[cfg(target_os = "windows")]
    fn try_hash() {
        let p = Path::new("target").join("debug").join("shapkg");
        assert_eq!(
            Command::new(p)
                .arg("hello-windows.yml")
                .output()
                .unwrap()
                .status
                .success(),
            true
        )
    }
    #[test]
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    fn try_hash() {
        let p = Path::new("target").join("debug").join("shapkg");
        assert_eq!(
            Command::new(p)
                .arg("hello-windows.yml")
                .output()
                .unwrap()
                .status
                .success(),
            true
        )
    }
}
