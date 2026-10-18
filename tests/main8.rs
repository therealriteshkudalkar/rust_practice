//! Tests for `src/main8.rs` — Counting Bits.

mod main8 {
    #![allow(dead_code, unused_imports, unused_mut)]

    include!("../src/main8.rs");

    #[cfg(test)]
    mod tests {
        use super::count_bits;

        #[test]
        fn zero_and_one() {
            assert_eq!(vec![0], count_bits(0));
            assert_eq!(vec![0, 1], count_bits(1));
        }

        #[test]
        fn counts_bits_up_to_five() {
            assert_eq!(vec![0, 1, 1, 2, 1, 2], count_bits(5));
        }

        #[test]
        fn counts_bits_up_to_eight() {
            assert_eq!(vec![0, 1, 1, 2, 1, 2, 2, 3, 1], count_bits(8));
        }

        #[test]
        fn length_is_n_plus_one() {
            let result = count_bits(16);
            assert_eq!(17, result.len());
            assert_eq!(1, result[16]);
            assert_eq!(4, result[15]);
        }
    }
}
