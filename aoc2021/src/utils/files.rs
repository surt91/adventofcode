#[macro_export]
macro_rules! data_str {
    ($value:expr) => {
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/", $value, ".dat"))
    };
}


