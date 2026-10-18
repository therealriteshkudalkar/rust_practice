//! Tests for `src/main58.rs` — Candy.

mod main58 {
    #![allow(dead_code, unused_imports, unused_mut)]

    include!("../src/main58.rs");

    #[cfg(test)]
    mod tests {
        use super::candy;

        #[test]
        fn example_one() {
            assert_eq!(5, candy(vec![1, 0, 2]));
        }

        #[test]
        fn example_two_equal_ratings_do_not_need_more_candy() {
            assert_eq!(4, candy(vec![1, 2, 2]));
        }

        #[test]
        fn example_three() {
            assert_eq!(9, candy(vec![1, 2, 2, 3, 2, 4]));
        }

        #[test]
        fn example_four() {
            assert_eq!(13, candy(vec![1, 2, 87, 87, 87, 2, 1]));
        }

        #[test]
        fn single_child_gets_one_candy() {
            assert_eq!(1, candy(vec![1]));
            assert_eq!(1, candy(vec![100]));
        }

        #[test]
        fn strictly_decreasing_ratings_need_increasing_candy() {
            assert_eq!(6, candy(vec![3, 2, 1]));
        }

        #[test]
        fn equal_ratings_all_get_one_candy() {
            assert_eq!(3, candy(vec![1, 1, 1]));
        }
    }
}
