//! Tests for `src/main45.rs` — Course Schedule.

mod main45 {
    #![allow(dead_code, unused_imports, unused_mut)]

    include!("../src/main45.rs");

    #[cfg(test)]
    mod tests {
        use super::can_finish;

        #[test]
        fn example_one() {
            assert!(can_finish(2, vec![vec![1, 0]]));
        }

        #[test]
        fn example_two_has_a_cycle() {
            assert!(!can_finish(2, vec![vec![1, 0], vec![0, 1]]));
        }

        #[test]
        fn diamond_dependencies_are_acyclic() {
            assert!(can_finish(
                4,
                vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]]
            ));
        }

        #[test]
        fn one_course_without_prerequisites() {
            assert!(can_finish(1, vec![]));
        }

        #[test]
        fn zero_courses_is_vacuously_finishable() {
            assert!(can_finish(0, vec![]));
        }

        #[test]
        fn three_course_cycle_is_impossible() {
            assert!(!can_finish(3, vec![vec![0, 1], vec![1, 2], vec![2, 0]]));
        }
    }
}
