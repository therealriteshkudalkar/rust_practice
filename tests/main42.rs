//! Tests for `src/main42.rs` — Number of Provinces.

mod main42 {
    #![allow(dead_code, unused_imports, unused_mut)]

    include!("../src/main42.rs");

    #[cfg(test)]
    mod tests {
        use super::find_circle_num;

        #[test]
        fn example_one() {
            assert_eq!(
                2,
                find_circle_num(vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]])
            );
        }

        #[test]
        fn example_two_every_city_is_its_own_province() {
            assert_eq!(
                3,
                find_circle_num(vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]])
            );
        }

        #[test]
        fn example_three_all_cities_are_connected() {
            assert_eq!(
                1,
                find_circle_num(vec![
                    vec![1, 0, 0, 1],
                    vec![0, 1, 1, 0],
                    vec![0, 1, 1, 1],
                    vec![1, 0, 1, 1],
                ])
            );
        }

        #[test]
        fn single_city_is_one_province() {
            assert_eq!(1, find_circle_num(vec![vec![1]]));
        }

        #[test]
        fn two_connected_cities_form_one_province() {
            assert_eq!(1, find_circle_num(vec![vec![1, 1], vec![1, 1]]));
        }
    }
}
