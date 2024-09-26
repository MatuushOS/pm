#[macro_export]
macro_rules! automate {
    ($action:expr) => {
        println!("Running {:#?} steps", $action);
        for i in &mut self.$action.0 {
            println!("\tRunning step {}", i.name);
            Command::new(&i.cmd[0])
                .args(&mut i.cmd[1..i.cmd.iter().len()])
                .output()?;
        }
    };
}
#[deprecated]
#[macro_export]
macro_rules! cli_automate {
    ($f:expr, $action:expr) => {
        if let Some(pm) = $f {
            for i in pm {
                let pkg = i.as_str();
                let b = crate::impls::Builder::default();
                b.$action(pkg).unwrap()
            }
        }
    };
}
