#[macro_export]
macro_rules! day_tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (a, b) = $name::run();
            let (sol_a, sol_b) = $value;
            assert_eq!(a, sol_a);
            assert_eq!(b, sol_b);
        }
    )*
    }
}
