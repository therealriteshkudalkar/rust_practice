//! Tests for `src/main52.rs` — Missing Number.

mod main52 {
    #![allow(dead_code, unused_imports, unused_mut)]

    include!("../src/main52.rs");

    #[cfg(test)]
    mod tests {
        use super::{missing_number, missing_number_with_xor_trick};

        #[test]
        fn example_one() {
            assert_eq!(2, missing_number(vec![3, 0, 1]));
            assert_eq!(2, missing_number_with_xor_trick(vec![3, 0, 1]));
        }

        #[test]
        fn example_two() {
            let nums = vec![9, 6, 4, 2, 3, 5, 7, 0, 1];
            assert_eq!(8, missing_number(nums.clone()));
            assert_eq!(8, missing_number_with_xor_trick(nums));
        }

        #[test]
        fn missing_value_is_the_upper_bound() {
            assert_eq!(2, missing_number(vec![0, 1]));
            assert_eq!(2, missing_number_with_xor_trick(vec![0, 1]));
        }

        #[test]
        fn missing_value_is_zero() {
            assert_eq!(0, missing_number(vec![1]));
            assert_eq!(0, missing_number_with_xor_trick(vec![1]));
        }

        #[test]
        fn both_implementations_agree_on_several_inputs() {
            let cases = vec![
                vec![3, 0, 1],
                vec![9, 6, 4, 2, 3, 5, 7, 0, 1],
                vec![0, 1],
                vec![1],
                vec![0],
                vec![4, 2, 3, 0, 1],
            ];
            for nums in cases {
                assert_eq!(
                    missing_number(nums.clone()),
                    missing_number_with_xor_trick(nums.clone()),
                    "nums = {nums:?}"
                );
            }
        }
    }
}
