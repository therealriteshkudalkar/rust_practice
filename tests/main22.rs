//! Tests for `src/main22.rs` — Transpose Matrix.

mod main22 {
    #![allow(dead_code, unused_imports, unused_mut)]

    include!("../src/main22.rs");

    #[cfg(test)]
    mod tests {
        use super::transpose;

        #[test]
        fn rectangular_matrix() {
            assert_eq!(
                vec![vec![1, 4], vec![2, 5], vec![3, 6]],
                transpose(&vec![vec![1, 2, 3], vec![4, 5, 6]])
            );
        }

        #[test]
        fn square_matrix() {
            assert_eq!(
                vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]],
                transpose(&vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]])
            );
        }

        #[test]
        fn single_row_becomes_single_column() {
            assert_eq!(
                vec![vec![1], vec![2], vec![3]],
                transpose(&vec![vec![1, 2, 3]])
            );
        }

        #[test]
        fn single_column_becomes_single_row() {
            assert_eq!(
                vec![vec![1, 2, 3]],
                transpose(&vec![vec![1], vec![2], vec![3]])
            );
        }

        #[test]
        fn transposing_twice_restores_the_original() {
            let matrix = vec![vec![1, 2, 3], vec![4, 5, 6]];
            assert_eq!(matrix, transpose(&transpose(&matrix)));
        }
    }
}
