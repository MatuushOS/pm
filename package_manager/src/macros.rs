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
