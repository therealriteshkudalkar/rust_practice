//! Tests for `src/main11.rs` — Divisor Game.
//!
//! Alice wins exactly when n is even (for the LeetCode constraint 1 <= n <= 1000).

mod main11 {
    #![allow(dead_code, unused_imports, unused_mut)]

    include!("../src/main11.rs");

    #[cfg(test)]
    mod tests {
        use super::divisor_game;

        #[test]
        fn one_is_a_losing_position() {
            assert!(!divisor_game(1));
        }

        #[test]
        fn two_is_a_winning_position() {
            assert!(divisor_game(2));
        }

        #[test]
        fn small_values_match_the_parity_rule() {
            for n in 1..=20 {
                assert_eq!(n % 2 == 0, divisor_game(n), "n = {n}");
            }
        }

        #[test]
        fn larger_values_match_the_parity_rule() {
            assert!(divisor_game(100));
            assert!(!divisor_game(99));
            assert!(divisor_game(1000));
        }
    }
}
