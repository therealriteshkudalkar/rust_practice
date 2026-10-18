//! Tests for `src/main55.rs` — Maximum Nesting Depth of the Parentheses.

mod main55 {
    #![allow(dead_code, unused_imports, unused_mut)]

    include!("../src/main55.rs");

    #[cfg(test)]
    mod tests {
        use super::max_depth;

        #[test]
        fn example_one() {
            assert_eq!(3, max_depth("(1+(2*3)+((8)/4))+1".to_string()));
        }

        #[test]
        fn example_two() {
            assert_eq!(3, max_depth("(1)+((2))+(((3)))".to_string()));
        }

        #[test]
        fn empty_and_single_character_strings_have_no_depth() {
            assert_eq!(0, max_depth(String::new()));
            assert_eq!(0, max_depth("1".to_string()));
        }

        #[test]
        fn flat_parentheses_have_depth_one() {
            assert_eq!(1, max_depth("()".to_string()));
            assert_eq!(1, max_depth("()()".to_string()));
            assert_eq!(1, max_depth("(a)(b)".to_string()));
        }

        #[test]
        fn nested_parentheses_increase_depth() {
            assert_eq!(3, max_depth("((()))".to_string()));
            assert_eq!(2, max_depth("(()())".to_string()));
        }
    }
}
