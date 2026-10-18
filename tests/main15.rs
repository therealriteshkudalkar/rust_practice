//! Tests for `src/main15.rs` — Best Time to Buy and Sell Stock.

mod main15 {
    #![allow(dead_code, unused_imports, unused_mut)]

    include!("../src/main15.rs");

    #[cfg(test)]
    mod tests {
        use super::{max_profit, max_profit_naive};

        #[test]
        fn example_one() {
            assert_eq!(5, max_profit(&vec![7, 1, 5, 3, 6, 4]));
        }

        #[test]
        fn example_two_has_no_profitable_pair() {
            assert_eq!(0, max_profit(&vec![7, 6, 4, 3, 1]));
        }

        #[test]
        fn single_day_has_no_profit() {
            assert_eq!(0, max_profit(&vec![5]));
        }

        #[test]
        fn flat_prices_have_no_profit() {
            assert_eq!(0, max_profit(&vec![3, 3, 3, 3]));
        }

        #[test]
        fn buy_at_first_then_sell_later() {
            assert_eq!(8, max_profit(&vec![2, 4, 1, 9]));
        }

        #[test]
        fn naive_and_linear_solutions_agree() {
            let cases = vec![
                vec![7, 1, 5, 3, 6, 4],
                vec![7, 6, 4, 3, 1],
                vec![8, 6, 9, 7, 8, 10, 3, 2, 4, 3, 12],
                vec![1, 2, 3, 4, 5],
                vec![5, 4, 3, 2, 1],
            ];
            for prices in cases {
                assert_eq!(
                    max_profit_naive(&prices),
                    max_profit(&prices),
                    "prices = {prices:?}"
                );
            }
        }
    }
}
