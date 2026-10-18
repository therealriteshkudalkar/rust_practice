//! Tests for `src/main30.rs` — Remove Element.

mod main30 {
    #![allow(dead_code, unused_imports, unused_mut)]

    include!("../src/main30.rs");

    #[cfg(test)]
    mod tests {
        use super::remove_element;

        #[test]
        fn example_one() {
            let mut nums = vec![3, 2, 2, 3];
            let k = remove_element(&mut nums, 3) as usize;
            assert_eq!(2, k);
            assert_eq!(vec![2, 2], nums[..k]);
        }

        #[test]
        fn example_two() {
            let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
            let k = remove_element(&mut nums, 2) as usize;
            assert_eq!(5, k);
            assert_eq!(vec![0, 1, 3, 0, 4], nums[..k]);
        }

        #[test]
        fn value_not_present_keeps_every_element() {
            let mut nums = vec![1, 2, 3];
            assert_eq!(3, remove_element(&mut nums, 9));
        }

        #[test]
        fn removing_every_element_returns_zero() {
            let mut nums = vec![2, 2, 2];
            assert_eq!(0, remove_element(&mut nums, 2));
        }

        #[test]
        fn empty_array_is_supported() {
            let mut nums: Vec<i32> = Vec::new();
            assert_eq!(0, remove_element(&mut nums, 1));
        }
    }
}
