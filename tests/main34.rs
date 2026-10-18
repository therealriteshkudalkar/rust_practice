//! Tests for `src/main34.rs` — Rotate Array.
//!
//! The source contains three implementations: `rotate` (one-by-one),
//! `rotate2` (pop/insert) and `rotate3` (reverse triple).

mod main34 {
    #![allow(dead_code, unused_imports, unused_mut)]

    include!("../src/main34.rs");

    #[cfg(test)]
    mod tests {
        use super::{rotate, rotate2, rotate3};

        const ORIGINAL: [i32; 7] = [1, 2, 3, 4, 5, 6, 7];

        #[test]
        fn example_one_all_implementations() {
            let expected = vec![5, 6, 7, 1, 2, 3, 4];

            let mut one = ORIGINAL.to_vec();
            rotate(&mut one, 3);
            assert_eq!(expected, one);

            let mut two = ORIGINAL.to_vec();
            rotate2(&mut two, 3);
            assert_eq!(expected, two);

            let mut three = ORIGINAL.to_vec();
            rotate3(&mut three, 3);
            assert_eq!(expected, three);
        }

        #[test]
        fn example_two_all_implementations() {
            let expected = vec![3, 99, -1, -100];

            let mut one = vec![-1, -100, 3, 99];
            rotate(&mut one, 2);
            assert_eq!(expected, one);

            let mut two = vec![-1, -100, 3, 99];
            rotate2(&mut two, 2);
            assert_eq!(expected, two);

            let mut three = vec![-1, -100, 3, 99];
            rotate3(&mut three, 2);
            assert_eq!(expected, three);
        }

        #[test]
        fn zero_rotations_leave_the_array_unchanged() {
            for implementation in 0..3 {
                let mut nums = ORIGINAL.to_vec();
                match implementation {
                    0 => rotate(&mut nums, 0),
                    1 => rotate2(&mut nums, 0),
                    _ => rotate3(&mut nums, 0),
                }
                assert_eq!(ORIGINAL.to_vec(), nums);
            }
        }

        #[test]
        fn full_rotation_leaves_the_array_unchanged() {
            for implementation in 0..3 {
                let mut nums = ORIGINAL.to_vec();
                match implementation {
                    0 => rotate(&mut nums, 7),
                    1 => rotate2(&mut nums, 7),
                    _ => rotate3(&mut nums, 7),
                }
                assert_eq!(ORIGINAL.to_vec(), nums);
            }
        }

        #[test]
        fn rotations_larger_than_the_length_wrap_around() {
            let expected = vec![3, 1, 2];
            for implementation in 0..3 {
                let mut nums = vec![1, 2, 3];
                match implementation {
                    0 => rotate(&mut nums, 4),
                    1 => rotate2(&mut nums, 4),
                    _ => rotate3(&mut nums, 4),
                }
                assert_eq!(expected, nums);
            }
        }

        #[test]
        fn empty_arrays_are_safe_for_the_safe_variants() {
            let mut nums: Vec<i32> = Vec::new();
            rotate2(&mut nums, 3);
            assert!(nums.is_empty());

            let mut nums: Vec<i32> = Vec::new();
            rotate3(&mut nums, 3);
            assert!(nums.is_empty());
        }
    }
}
