//! Tests for `src/main27.rs` — Maximum Product Difference Between Two Pairs.

mod main27 {
    #![allow(dead_code, unused_imports, unused_mut)]

    include!("../src/main27.rs");

    #[cfg(test)]
    mod tests {
        use super::max_product_difference;

        #[test]
        fn example_one() {
            assert_eq!(34, max_product_difference(&vec![5, 6, 2, 7, 4]));
        }

        #[test]
        fn example_two() {
            assert_eq!(64, max_product_difference(&vec![4, 2, 5, 9, 7, 4, 8]));
        }

        #[test]
        fn all_equal_values_give_zero() {
            assert_eq!(0, max_product_difference(&vec![1, 1, 1, 1]));
        }

        #[test]
        fn negative_values_are_supported() {
            // Largest pair: -1 * -2 = 2; smallest pair: -3 * -4 = 12; 2 - 12 = -10.
            assert_eq!(-10, max_product_difference(&vec![-1, -2, -3, -4]));
        }

        #[test]
        fn two_values_cannot_form_two_disjoint_pairs() {
            assert_eq!(0, max_product_difference(&vec![1, 3]));
        }
    }
}
