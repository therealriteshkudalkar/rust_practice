//! Tests for `src/main29.rs` — Merge Sorted Array.

mod main29 {
    #![allow(dead_code, unused_imports, unused_mut)]

    include!("../src/main29.rs");

    #[cfg(test)]
    mod tests {
        use super::merge;

        #[test]
        fn example_one() {
            let mut nums1 = vec![1, 2, 3, 0, 0, 0];
            let mut nums2 = vec![2, 5, 6];
            merge(&mut nums1, 3, &mut nums2, 3);
            assert_eq!(vec![1, 2, 2, 3, 5, 6], nums1);
        }

        #[test]
        fn second_array_is_empty() {
            let mut nums1 = vec![1];
            let mut nums2 = vec![];
            merge(&mut nums1, 1, &mut nums2, 0);
            assert_eq!(vec![1], nums1);
        }

        #[test]
        fn first_array_has_no_real_elements() {
            let mut nums1 = vec![0];
            let mut nums2 = vec![1];
            merge(&mut nums1, 0, &mut nums2, 1);
            assert_eq!(vec![1], nums1);
        }

        #[test]
        fn duplicates_across_arrays_are_preserved() {
            let mut nums1 = vec![1, 1, 0, 0];
            let mut nums2 = vec![1, 1];
            merge(&mut nums1, 2, &mut nums2, 2);
            assert_eq!(vec![1, 1, 1, 1], nums1);
        }

        #[test]
        fn negative_values_are_sorted_correctly() {
            let mut nums1 = vec![-3, -1, 0, 0, 0];
            let mut nums2 = vec![-2, 4, 5];
            merge(&mut nums1, 2, &mut nums2, 3);
            assert_eq!(vec![-3, -2, -1, 4, 5], nums1);
        }
    }
}
