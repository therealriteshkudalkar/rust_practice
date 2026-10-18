//! Tests for `src/main36.rs` — Jump Game.

mod main36 {
    #![allow(dead_code, unused_imports, unused_mut)]

    include!("../src/main36.rs");

    #[cfg(test)]
    mod tests {
        use super::can_jump;

        #[test]
        fn example_one() {
            assert!(can_jump(vec![2, 3, 1, 1, 4]));
        }

        #[test]
        fn example_two() {
            assert!(!can_jump(vec![3, 2, 1, 0, 4]));
        }

        #[test]
        fn zero_in_the_middle_can_block_the_path() {
            assert!(!can_jump(vec![2, 2, 1, 0, 4]));
        }

        #[test]
        fn single_step_is_always_reachable() {
            assert!(can_jump(vec![0]));
            assert!(can_jump(vec![1]));
        }

        #[test]
        fn jumps_can_skip_over_zeroes() {
            assert!(can_jump(vec![2, 0, 0]));
        }
    }
}
