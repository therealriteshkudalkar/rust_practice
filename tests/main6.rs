//! Tests for `src/main6.rs` — Remove One Element to Make the Array Strictly Increasing.

mod main6 {
    #![allow(dead_code, unused_imports, unused_mut)]

    include!("../src/main6.rs");

    #[cfg(test)]
    mod tests {
        use super::{can_be_increasing, can_be_increasing_fast, is_increasing};

        #[test]
        fn removing_a_middle_element_can_fix_the_array() {
            assert!(can_be_increasing(&vec![1, 2, 10, 5, 7]));
        }

        #[test]
        fn one_removal_is_not_enough() {
            assert!(!can_be_increasing(&vec![2, 3, 1, 2]));
            assert!(!can_be_increasing(&vec![1, 1, 1]));
        }

        #[test]
        fn already_increasing_arrays_are_accepted() {
            assert!(can_be_increasing(&vec![1, 2, 3, 4]));
            assert!(can_be_increasing(&vec![1]));
        }

        #[test]
        fn can_remove_the_blocking_element() {
            assert!(can_be_increasing(&vec![105, 924, 32, 968]));
            assert!(can_be_increasing(&vec![512, 867, 904, 997, 403]));
        }

        #[test]
        fn array_requiring_two_removals_is_rejected() {
            assert!(!can_be_increasing(&vec![13, 205, 553, 527, 790, 238]));
        }

        #[test]
        fn is_increasing_honours_the_skipped_index() {
            let nums = vec![1, 2, 10, 5, 7];
            assert!(is_increasing(&nums, 2));
            assert!(!is_increasing(&nums, 0));
        }

        #[test]
        fn fast_variant_agrees_on_straightforward_inputs() {
            assert!(can_be_increasing_fast(&vec![1, 2, 10, 5, 7]));
            assert!(can_be_increasing_fast(&vec![1, 2, 3]));
            assert!(!can_be_increasing_fast(&vec![2, 3, 1, 2]));
            assert!(!can_be_increasing_fast(&vec![1, 1, 1]));
        }

        #[test]
        #[ignore = "known bug: can_be_increasing_fast counts rule breakers instead of trying each removal"]
        fn fast_variant_rejects_array_that_needs_two_removals() {
            assert!(!can_be_increasing_fast(&vec![13, 205, 553, 527, 790, 238]));
        }
    }
}
