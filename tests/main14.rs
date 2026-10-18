//! Tests for `src/main14.rs` — Pascal's Triangle II.

mod main14 {
    #![allow(dead_code, unused_imports, unused_mut)]

    include!("../src/main14.rs");

    #[cfg(test)]
    mod tests {
        use super::get_row;

        #[test]
        fn row_zero() {
            assert_eq!(vec![1], get_row(0));
        }

        #[test]
        fn row_one() {
            assert_eq!(vec![1, 1], get_row(1));
        }

        #[test]
        fn row_three() {
            assert_eq!(vec![1, 3, 3, 1], get_row(3));
        }

        #[test]
        fn row_four() {
            assert_eq!(vec![1, 4, 6, 4, 1], get_row(4));
        }

        #[test]
        fn row_ten() {
            assert_eq!(
                vec![1, 10, 45, 120, 210, 252, 210, 120, 45, 10, 1],
                get_row(10)
            );
        }

        #[test]
        fn every_row_is_symmetric_and_starts_and_ends_with_one() {
            for row_index in 0..=15 {
                let row = get_row(row_index);
                assert_eq!(row_index as usize + 1, row.len());
                assert_eq!(1, row[0]);
                assert_eq!(1, row[row.len() - 1]);
                let mut reversed = row.clone();
                reversed.reverse();
                assert_eq!(row, reversed);
            }
        }
    }
}
