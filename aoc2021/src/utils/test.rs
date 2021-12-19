#[macro_export]
macro_rules! test {
    (#$name:ident: $value:expr) => {
        #[test]
        #[ignore]
        fn $name() {
            let (a, b) = $name::run();
            let (sol_a, sol_b) = $value;
            assert_eq!(a, sol_a);
            assert_eq!(b, sol_b);
        }
    };

    ($name:ident: $value:expr) => {
        #[test]
        fn $name() {
            let (a, b) = $name::run();
            let (sol_a, sol_b) = $value;
            assert_eq!(a, sol_a);
            assert_eq!(b, sol_b);
        }
    };
}
