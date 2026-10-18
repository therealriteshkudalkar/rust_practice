//! Tests for `src/main38.rs` — H-Index.

mod main38 {
    #![allow(dead_code, unused_imports, unused_mut)]

    include!("../src/main38.rs");

    #[cfg(test)]
    mod tests {
        use super::h_index;

        #[test]
        fn example_one() {
            assert_eq!(3, h_index(vec![3, 0, 6, 1, 5]));
        }

        #[test]
        fn example_two() {
            assert_eq!(1, h_index(vec![1, 3, 1]));
        }

        #[test]
        fn single_highly_cited_paper() {
            assert_eq!(1, h_index(vec![100]));
        }

        #[test]
        fn single_uncited_paper() {
            assert_eq!(0, h_index(vec![0]));
        }

        #[test]
        fn four_papers_with_at_least_four_citations_each() {
            assert_eq!(4, h_index(vec![7, 10, 10, 10]));
        }

        #[test]
        fn no_citations_means_zero() {
            assert_eq!(0, h_index(vec![0, 0, 0]));
        }

        #[test]
        fn h_index_cannot_exceed_the_number_of_papers() {
            for citations in [vec![100, 100], vec![1, 2, 3], vec![5, 5, 5, 5, 5]] {
                let h = h_index(citations.clone());
                assert!(h <= citations.len() as i32);
            }
        }
    }
}
