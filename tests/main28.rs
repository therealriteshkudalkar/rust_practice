//! Tests for `src/main28.rs` — Kids With the Greatest Number of Candies.

mod main28 {
    #![allow(dead_code, unused_imports, unused_mut)]

    include!("../src/main28.rs");

    #[cfg(test)]
    mod tests {
        use super::kids_with_candies;

        #[test]
        fn example_one() {
            assert_eq!(
                vec![true, true, true, false, true],
                kids_with_candies(vec![2, 3, 5, 1, 3], 3)
            );
        }

        #[test]
        fn example_two() {
            assert_eq!(
                vec![true, false, false, false, false],
                kids_with_candies(vec![4, 2, 1, 1, 2], 1)
            );
        }

        #[test]
        fn example_three() {
            assert_eq!(
                vec![true, false, true],
                kids_with_candies(vec![12, 1, 12], 10)
            );
        }

        #[test]
        fn zero_extra_candies_marks_only_the_current_maximum() {
            assert_eq!(
                vec![false, true, false],
                kids_with_candies(vec![1, 3, 2], 0)
            );
        }

        #[test]
        fn equal_candies_using_the_extra_candy_still_count() {
            assert_eq!(vec![true, true, true], kids_with_candies(vec![1, 1, 1], 0));
        }
    }
}
