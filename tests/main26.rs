//! Tests for `src/main26.rs` — Contains Duplicate II.

mod main26 {
    #![allow(dead_code, unused_imports, unused_mut)]

    include!("../src/main26.rs");

    #[cfg(test)]
    mod tests {
        use super::contains_nearby_duplicate;

        #[test]
        fn example_one() {
            assert!(contains_nearby_duplicate(&vec![1, 2, 3, 1], 3));
        }

        #[test]
        fn example_two() {
            assert!(contains_nearby_duplicate(&vec![1, 0, 1, 1], 1));
        }

        #[test]
        fn example_three() {
            assert!(!contains_nearby_duplicate(&vec![1, 2, 3, 1, 2, 3], 1));
        }

        #[test]
        fn no_duplicates_at_all() {
            assert!(!contains_nearby_duplicate(&vec![1, 2, 3, 4], 3));
        }

        #[test]
        fn zero_distance_never_matches() {
            assert!(!contains_nearby_duplicate(&vec![1, 1], 0));
        }

        #[test]
        fn distance_is_inclusive() {
            assert!(contains_nearby_duplicate(&vec![1, 2, 1], 2));
            assert!(!contains_nearby_duplicate(&vec![1, 2, 3, 1], 2));
        }
    }
}
