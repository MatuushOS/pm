#[macro_export]
macro_rules! cli_automate {
    ($f:expr, $p:block) => {
        if let Some(pm) = $f {
            for i in pm {
                $p
            }
        }
    };
}
