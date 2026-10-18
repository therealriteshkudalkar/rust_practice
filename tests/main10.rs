//! Tests for `src/main10.rs` — Fibonacci Number.

mod main10 {
    #![allow(dead_code, unused_imports, unused_mut)]

    include!("../src/main10.rs");

    #[cfg(test)]
    mod tests {
        use super::fib;

        #[test]
        fn base_cases() {
            assert_eq!(0, fib(0));
            assert_eq!(1, fib(1));
        }

        #[test]
        fn small_values() {
            assert_eq!(1, fib(2));
            assert_eq!(2, fib(3));
            assert_eq!(3, fib(4));
            assert_eq!(5, fib(5));
            assert_eq!(8, fib(6));
        }

        #[test]
        fn classic_example() {
            assert_eq!(55, fib(10));
        }

        #[test]
        fn largest_leetcode_input_fits_in_i32() {
            assert_eq!(1_134_903_170, fib(45));
        }

        #[test]
        fn consecutive_values_satisfy_the_recurrence() {
            let mut previous = fib(0);
            let mut current = fib(1);
            for n in 2..=20 {
                let next = fib(n);
                assert_eq!(previous + current, next);
                previous = current;
                current = next;
            }
        }
    }
}
