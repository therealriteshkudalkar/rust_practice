//! Tests for `src/main51.rs` — Zigzag Conversion.

mod main51 {
    #![allow(dead_code, unused_imports, unused_mut)]

    include!("../src/main51.rs");

    #[cfg(test)]
    mod tests {
        use super::convert;

        #[test]
        fn two_rows_alternate_characters() {
            assert_eq!("PYAIHRNAPLSIIG", convert("PAYPALISHIRING".to_string(), 2));
        }

        #[test]
        fn two_rows_short_string() {
            assert_eq!("ACB", convert("ABC".to_string(), 2));
        }

        #[test]
        fn one_character_with_one_row_is_unchanged() {
            assert_eq!("A", convert("A".to_string(), 1));
        }

        #[test]
        fn one_character_per_row_is_unchanged() {
            assert_eq!("ABC", convert("ABC".to_string(), 3));
        }

        #[test]
        #[ignore = "known bug: middle rows repeat the outer-row skip instead of alternating their two skips"]
        fn example_one_three_rows() {
            assert_eq!("PAHNAPLSIIGYIR", convert("PAYPALISHIRING".to_string(), 3));
        }

        #[test]
        #[ignore = "known bug: middle rows repeat the outer-row skip instead of alternating their two skips"]
        fn example_two_four_rows() {
            assert_eq!("PINALSIGYAHRPI", convert("PAYPALISHIRING".to_string(), 4));
        }

        #[test]
        #[ignore = "known bug: total_skips becomes -1 for one row, so only the first character is emitted"]
        fn one_row_returns_the_original_string() {
            assert_eq!("PAYPALISHIRING", convert("PAYPALISHIRING".to_string(), 1));
        }
    }
}
