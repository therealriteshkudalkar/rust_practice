//! Tests for `src/main3.rs` — Valid Parentheses.

mod main3 {
    #![allow(dead_code, unused_imports, unused_mut)]

    include!("../src/main3.rs");

    #[cfg(test)]
    mod tests {
        use super::{are_matching_brace, is_closing_brace, is_opening_brace, is_valid};

        #[test]
        fn accepts_single_pairs() {
            assert!(is_valid("()".to_string()));
            assert!(is_valid("[]".to_string()));
            assert!(is_valid("{}".to_string()));
        }

        #[test]
        fn accepts_concatenated_pairs() {
            assert!(is_valid("()[]{}".to_string()));
        }

        #[test]
        fn accepts_nested_pairs() {
            assert!(is_valid("{[()]}".to_string()));
            assert!(is_valid("(([]){})".to_string()));
        }

        #[test]
        fn rejects_mismatched_pairs() {
            assert!(!is_valid("(]".to_string()));
            assert!(!is_valid("([)]".to_string()));
            assert!(!is_valid("(}".to_string()));
        }

        #[test]
        fn rejects_unclosed_and_unopened_brackets() {
            assert!(!is_valid("(((".to_string()));
            assert!(!is_valid(")".to_string()));
            assert!(!is_valid("(()".to_string()));
        }

        #[test]
        fn empty_string_is_valid() {
            assert!(is_valid(String::new()));
        }

        #[test]
        fn brace_classification_helpers_agree() {
            for ch in ['(', '[', '{'] {
                assert!(is_opening_brace(ch));
                assert!(!is_closing_brace(ch));
            }
            for ch in [')', ']', '}'] {
                assert!(is_closing_brace(ch));
                assert!(!is_opening_brace(ch));
            }
            assert!(are_matching_brace('(', ')'));
            assert!(are_matching_brace('[', ']'));
            assert!(are_matching_brace('{', '}'));
            assert!(!are_matching_brace('(', ']'));
            assert!(!are_matching_brace('{', ')'));
        }
    }
}
