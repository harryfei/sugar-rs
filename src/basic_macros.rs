#[macro_export]
macro_rules! cmp {

    // binary operator
    ($a:expr, < $b:expr) => { $a < $b };
    ($a:expr, <= $b:expr) => { $a <= $b };
    ($a:expr, > $b:expr) => { $a > $b };
    ($a:expr, >= $b:expr) => { $a >= $b };
    ($a:expr, == $b:expr) => { $a == $b };
    ($a:expr, != $b:expr) => { $a != $b };

    // multiple members operator (members >= 3)
    ($a:expr, $op1:tt $b:expr, $($op:tt $e:expr),+) => {{
        let b = $b;
        cmp!($a, $op1 b) && cmp!(b, $($op $e),+)
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn cmp_3_test() {
        assert!(cmp!(3, > 2, > 1));
        assert!(cmp!(1, < 2, < 3));
        assert!(cmp!(3, > 2, < 4));
        assert!(cmp!(4, > 2, < 3));
        assert!(cmp!(3, >= 3, > 2));
        assert!(cmp!(3, > 2, >= 2));
        assert!(cmp!(3, >= 3, <= 3));
        assert!(cmp!(2, == 2, == 2));
        assert!(cmp!(1, != 2, == 2));
    }

    #[test]
    fn cmp_multiple_test() {
        assert!(cmp!(1, < 2, == 2, <= 3, < 4));
        assert!(cmp!(1, != 2, == 2, > 1, > 0));
    }
}
