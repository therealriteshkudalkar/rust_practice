//! Tests for `src/main31.rs` — Remove Duplicates from Sorted Array.

mod main31 {
    #![allow(dead_code, unused_imports, unused_mut)]

    include!("../src/main31.rs");

    #[cfg(test)]
    mod tests {
        use super::remove_duplicates;

        #[test]
        fn example_one() {
            let mut nums = vec![1, 1, 2];
            let k = remove_duplicates(&mut nums) as usize;
            assert_eq!(2, k);
            assert_eq!(vec![1, 2], nums[..k]);
        }

        #[test]
        fn example_two() {
            let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
            let k = remove_duplicates(&mut nums) as usize;
            assert_eq!(5, k);
            assert_eq!(vec![0, 1, 2, 3, 4], nums[..k]);
        }

        #[test]
        fn single_element_is_unchanged() {
            let mut nums = vec![1];
            assert_eq!(1, remove_duplicates(&mut nums));
        }

        #[test]
        fn all_elements_equal_keep_one() {
            let mut nums = vec![2, 2, 2, 2];
            let k = remove_duplicates(&mut nums) as usize;
            assert_eq!(1, k);
            assert_eq!(vec![2], nums[..k]);
        }

        #[test]
        fn already_unique_array_is_unchanged() {
            let mut nums = vec![1, 2, 3, 4];
            let k = remove_duplicates(&mut nums) as usize;
            assert_eq!(4, k);
            assert_eq!(vec![1, 2, 3, 4], nums[..k]);
        }
    }
}
