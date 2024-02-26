#[macro_export]
macro_rules! norm {
    ($a:expr, $b:expr) => {
        ((($a * $a + $b * $b) as f64).sqrt())
    };
}
