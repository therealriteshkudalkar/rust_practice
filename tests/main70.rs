//! Tests for `src/main70.rs` — Relative Ranks.

mod main70 {
    #![allow(dead_code, unused_imports, unused_mut)]

    include!("../src/main70.rs");

    #[cfg(test)]
    mod tests {
        use super::find_relative_ranks;

        fn strings(values: &[&str]) -> Vec<String> {
            values.iter().map(|value| value.to_string()).collect()
        }

        #[test]
        fn example_one() {
            assert_eq!(
                strings(&["Gold Medal", "Silver Medal", "Bronze Medal", "4", "5"]),
                find_relative_ranks(vec![5, 4, 3, 2, 1])
            );
        }

        #[test]
        fn example_two() {
            assert_eq!(
                strings(&["Gold Medal", "5", "Bronze Medal", "Silver Medal", "4"]),
                find_relative_ranks(vec![10, 3, 8, 9, 4])
            );
        }

        #[test]
        fn single_competitor_wins_gold() {
            assert_eq!(strings(&["Gold Medal"]), find_relative_ranks(vec![1]));
        }

        #[test]
        fn two_competitors() {
            assert_eq!(
                strings(&["Silver Medal", "Gold Medal"]),
                find_relative_ranks(vec![1, 2])
            );
        }

        #[test]
        fn ranks_are_assigned_by_sorted_score() {
            assert_eq!(
                strings(&["4", "Gold Medal", "Bronze Medal", "Silver Medal"]),
                find_relative_ranks(vec![4, 10, 8, 9])
            );
        }
    }
}
