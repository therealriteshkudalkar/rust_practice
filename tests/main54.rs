//! Tests for `src/main54.rs` — Text Justification.

mod main54 {
    #![allow(dead_code, unused_imports, unused_mut)]

    include!("../src/main54.rs");

    #[cfg(test)]
    mod tests {
        use super::{full_justify, justify_line};

        fn strings(values: &[&str]) -> Vec<String> {
            values.iter().map(|value| value.to_string()).collect()
        }

        #[test]
        fn justify_line_distributes_extra_spaces() {
            assert_eq!(
                "This    is    an",
                justify_line(strings(&["This", "is", "an"]), 10, 16)
            );
        }

        #[test]
        fn justify_line_pads_a_single_word() {
            assert_eq!("word      ", justify_line(strings(&["word"]), 4, 10));
        }

        #[test]
        fn full_justify_example_one() {
            let words = strings(&[
                "This",
                "is",
                "an",
                "example",
                "of",
                "text",
                "justification.",
            ]);
            assert_eq!(
                strings(&["This    is    an", "example  of text", "justification.  "]),
                full_justify(words, 16)
            );
        }

        #[test]
        fn full_justify_example_two() {
            let words = strings(&["What", "must", "be", "acknowledgment", "shall", "be"]);
            assert_eq!(
                strings(&["What   must   be", "acknowledgment  ", "shall be        "]),
                full_justify(words, 16)
            );
        }

        #[test]
        fn last_line_is_left_justified() {
            assert_eq!(strings(&["a b  "]), full_justify(strings(&["a", "b"]), 5));
        }

        #[test]
        fn single_word_is_padded_to_the_full_width() {
            assert_eq!(strings(&["a    "]), full_justify(strings(&["a"]), 5));
        }
    }
}
