//! Tests for `src/main50.rs` — Longest Common Prefix.

mod main50 {
    #![allow(dead_code, unused_imports, unused_mut)]

    include!("../src/main50.rs");

    #[cfg(test)]
    mod tests {
        use super::longest_common_prefix;

        fn strings(values: &[&str]) -> Vec<String> {
            values.iter().map(|value| value.to_string()).collect()
        }

        #[test]
        fn example_one() {
            assert_eq!(
                "fl",
                longest_common_prefix(strings(&["flower", "flow", "flight"]))
            );
        }

        #[test]
        fn example_two_has_no_common_prefix() {
            assert_eq!("", longest_common_prefix(strings(&["dog", "car", "racer"])));
        }

        #[test]
        fn example_three() {
            assert_eq!(
                "man",
                longest_common_prefix(strings(&["man", "manipulative", "mandarin"]))
            );
        }

        #[test]
        fn shared_prefix_longer_than_one_character() {
            assert_eq!(
                "inters",
                longest_common_prefix(strings(&["interspecies", "interstellar", "interstate"]))
            );
        }

        #[test]
        fn empty_string_in_the_input_makes_the_prefix_empty() {
            assert_eq!("", longest_common_prefix(strings(&["", "abc"])));
        }

        #[test]
        fn single_string_is_its_own_prefix() {
            assert_eq!("abc", longest_common_prefix(strings(&["abc"])));
        }

        #[test]
        fn shorter_string_can_be_the_whole_prefix() {
            assert_eq!("a", longest_common_prefix(strings(&["ab", "a"])));
        }

        #[test]
        fn empty_input_returns_empty_prefix() {
            assert_eq!("", longest_common_prefix(Vec::new()));
        }
    }
}
