//! Tests for `src/main39.rs` — Product of Array Except Self.

mod main39 {
    #![allow(dead_code, unused_imports, unused_mut)]

    include!("../src/main39.rs");

    #[cfg(test)]
    mod tests {
        use super::product_except_self;

        #[test]
        fn example_one() {
            assert_eq!(vec![24, 12, 8, 6], product_except_self(vec![1, 2, 3, 4]));
        }

        #[test]
        fn example_two_contains_one_zero() {
            assert_eq!(
                vec![0, 0, 9, 0, 0],
                product_except_self(vec![-1, 1, 0, -3, 3])
            );
        }

        #[test]
        fn two_zeroes_make_every_product_zero() {
            assert_eq!(vec![0, 0], product_except_self(vec![0, 0]));
            assert_eq!(vec![0, 0, 0], product_except_self(vec![0, 4, 0]));
        }

        #[test]
        fn two_elements_swap_their_values() {
            assert_eq!(vec![3, 2], product_except_self(vec![2, 3]));
        }

        #[test]
        fn single_element_has_an_empty_product() {
            assert_eq!(vec![1], product_except_self(vec![7]));
        }

        #[test]
        fn negative_values_keep_their_signs() {
            assert_eq!(vec![6, -3, -2], product_except_self(vec![-1, 2, 3]));
        }
    }
}
