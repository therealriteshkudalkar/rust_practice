//! Tests for `src/main62.rs` — Pow(x, n).

mod main62 {
    #![allow(dead_code, unused_imports, unused_mut)]

    include!("../src/main62.rs");

    #[cfg(test)]
    mod tests {
        use super::my_pow;

        fn assert_close(expected: f64, actual: f64) {
            assert!(
                (expected - actual).abs() < 1e-9,
                "expected {expected}, got {actual}"
            );
        }

        #[test]
        fn positive_exponent() {
            assert_close(1024.0, my_pow(2.0, 10));
            assert_close(9.261000000000001, my_pow(2.1, 3));
        }

        #[test]
        fn zero_exponent_is_one() {
            assert_close(1.0, my_pow(2.0, 0));
            assert_close(1.0, my_pow(0.0, 0));
        }

        #[test]
        fn negative_exponent_uses_the_reciprocal() {
            assert_close(0.25, my_pow(2.0, -2));
            assert_close(0.125, my_pow(2.0, -3));
        }

        #[test]
        fn negative_base() {
            assert_close(-8.0, my_pow(-2.0, 3));
            assert_close(16.0, my_pow(-2.0, 4));
            assert_close(-0.125, my_pow(-2.0, -3));
        }

        #[test]
        fn trivial_bases() {
            assert_close(0.0, my_pow(0.0, 5));
            assert_close(1.0, my_pow(1.0, -1000));
        }

        #[test]
        fn large_positive_exponent() {
            assert_close(2_147_483_648.0, my_pow(2.0, 31));
        }
    }
}
