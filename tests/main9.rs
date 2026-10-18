//! Tests for `src/main9.rs` — Pascal's Triangle.

mod main9 {
    #![allow(dead_code, unused_imports, unused_mut)]

    include!("../src/main9.rs");

    #[cfg(test)]
    mod tests {
        use super::generate;

        #[test]
        fn one_row() {
            assert_eq!(vec![vec![1]], generate(1));
        }

        #[test]
        fn two_rows() {
            assert_eq!(vec![vec![1], vec![1, 1]], generate(2));
        }

        #[test]
        fn five_rows() {
            assert_eq!(
                vec![
                    vec![1],
                    vec![1, 1],
                    vec![1, 2, 1],
                    vec![1, 3, 3, 1],
                    vec![1, 4, 6, 4, 1],
                ],
                generate(5)
            );
        }

        #[test]
        fn rows_have_the_expected_shape_and_symmetry() {
            let triangle = generate(10);
            assert_eq!(10, triangle.len());
            for (index, row) in triangle.iter().enumerate() {
                assert_eq!(index + 1, row.len());
                assert_eq!(1, *row.first().unwrap());
                assert_eq!(1, *row.last().unwrap());
                let reversed: Vec<i32> = row.iter().rev().copied().collect();
                assert_eq!(*row, reversed);
            }
        }
    }
}
