#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::process::Command;

    #[test]
    #[cfg(target_os = "windows")]
    fn check_install_windows() {
        assert!(Command::new(Path::new(
            &Path::new("target").join("debug").join("pm")
        ))
        .args(["-i", "tests/hello-windows.yml"])
        .status()
        .unwrap()
        .success())
    }
    #[test]
    #[cfg(any(target_os = "macos", target_os = "linux"))]
    fn check_install_macos_unix_linux() {
        assert!(Command::new(Path::new(
            &Path::new("target").join("debug").join("pm")
        ))
        .args(["-i", "tests/hello-unix.yml"])
        .status()
        .unwrap()
        .success())
    }
}
