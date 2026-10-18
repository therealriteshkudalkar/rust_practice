//! Tests for `src/main60.rs` — Naming a Company.

mod main60 {
    #![allow(dead_code, unused_imports, unused_mut)]

    include!("../src/main60.rs");

    #[cfg(test)]
    mod tests {
        use super::{distinct_names, distinct_names_optimized};

        fn ideas(values: &[&str]) -> Vec<String> {
            values.iter().map(|value| value.to_string()).collect()
        }

        #[test]
        fn example_one() {
            let input = ideas(&["coffee", "donuts", "time", "toffee"]);
            assert_eq!(6, distinct_names_optimized(input));
        }

        #[test]
        fn example_two_same_suffixes_after_swap() {
            let input = ideas(&["lack", "back"]);
            assert_eq!(0, distinct_names_optimized(input));
        }

        #[test]
        fn example_three() {
            let input = ideas(&["alrgtxxdj", "illqfngl", "rlrgtxxdj"]);
            assert_eq!(4, distinct_names_optimized(input));
        }

        #[test]
        fn two_unrelated_names_produce_two_companies() {
            let input = ideas(&["abc", "def"]);
            assert_eq!(2, distinct_names_optimized(input));
        }

        #[test]
        fn naive_version_agrees_with_optimized_version() {
            let cases = vec![
                ideas(&["coffee", "donuts", "time", "toffee"]),
                ideas(&["lack", "back"]),
                ideas(&["alrgtxxdj", "illqfngl", "rlrgtxxdj"]),
                ideas(&["abc", "def"]),
                ideas(&["aa", "ab"]),
            ];
            for input in cases {
                assert_eq!(
                    distinct_names_optimized(input.clone()),
                    distinct_names(input.clone()),
                    "ideas = {input:?}"
                );
            }
        }
    }
}
