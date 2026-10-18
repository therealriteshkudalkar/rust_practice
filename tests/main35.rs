//! Tests for `src/main35.rs` — Best Time to Buy and Sell Stock II.

mod main35 {
    #![allow(dead_code, unused_imports, unused_mut)]

    include!("../src/main35.rs");

    #[cfg(test)]
    mod tests {
        use super::max_profit;

        #[test]
        fn example_one() {
            assert_eq!(7, max_profit(vec![7, 1, 5, 3, 6, 4]));
        }

        #[test]
        fn example_two_is_strictly_increasing() {
            assert_eq!(4, max_profit(vec![1, 2, 3, 4, 5]));
        }

        #[test]
        fn decreasing_prices_yield_no_profit() {
            assert_eq!(0, max_profit(vec![7, 6, 4, 3, 1]));
        }

        #[test]
        fn single_price_has_no_profit() {
            assert_eq!(0, max_profit(vec![5]));
        }

        #[test]
        fn flat_prices_have_no_profit() {
            assert_eq!(0, max_profit(vec![2, 2, 2]));
        }

        #[test]
        fn profit_is_taken_on_every_upward_step() {
            assert_eq!(1, max_profit(vec![1, 2]));
            assert_eq!(3, max_profit(vec![1, 2, 3, 4]));
        }
    }
}
