//! Tests for `src/main37.rs` — Jump Game II.

mod main37 {
    #![allow(dead_code, unused_imports, unused_mut)]

    include!("../src/main37.rs");

    #[cfg(test)]
    mod tests {
        use super::jump;

        #[test]
        fn example_one() {
            assert_eq!(2, jump(vec![2, 3, 1, 1, 4]));
        }

        #[test]
        fn example_two() {
            assert_eq!(2, jump(vec![2, 3, 0, 1, 4]));
        }

        #[test]
        fn already_at_the_last_index() {
            assert_eq!(0, jump(vec![0]));
            assert_eq!(0, jump(vec![5]));
        }

        #[test]
        fn can_reach_the_end_in_one_jump() {
            assert_eq!(1, jump(vec![3, 0, 0, 0]));
            assert_eq!(1, jump(vec![1, 0]));
        }

        #[test]
        fn forced_one_step_at_a_time() {
            assert_eq!(4, jump(vec![1, 1, 1, 1, 1]));
        }
    }
}
