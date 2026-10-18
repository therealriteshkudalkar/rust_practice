//! Tests for `src/main71.rs` — Maximum Product of Two Elements in an Array.

mod main71 {
    #![allow(dead_code, unused_imports, unused_mut)]

    include!("../src/main71.rs");

    #[cfg(test)]
    mod tests {
        use super::max_product;

        #[test]
        fn example_one() {
            assert_eq!(12, max_product(vec![3, 4, 5, 2]));
        }

        #[test]
        fn example_two() {
            assert_eq!(16, max_product(vec![1, 5, 4, 5]));
        }

        #[test]
        fn example_three_only_two_values() {
            assert_eq!(12, max_product(vec![3, 7]));
        }

        #[test]
        fn equal_values() {
            assert_eq!(4, max_product(vec![3, 3]));
            assert_eq!(0, max_product(vec![1, 1]));
        }

        #[test]
        fn largest_values_may_be_at_the_end() {
            assert_eq!(72, max_product(vec![1, 2, 10, 9]));
        }
    }
}
