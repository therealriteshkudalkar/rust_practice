//! Tests for `src/main40.rs` — Gas Station.

mod main40 {
    #![allow(dead_code, unused_imports, unused_mut)]

    include!("../src/main40.rs");

    #[cfg(test)]
    mod tests {
        use super::{can_complete_circuit, can_complete_circuit_naive};

        #[test]
        fn example_one() {
            let gas = vec![1, 2, 3, 4, 5];
            let cost = vec![3, 4, 5, 1, 2];
            assert_eq!(3, can_complete_circuit(gas.clone(), cost.clone()));
            assert_eq!(3, can_complete_circuit_naive(gas, cost));
        }

        #[test]
        fn example_two_is_impossible() {
            let gas = vec![2, 3, 4];
            let cost = vec![3, 4, 3];
            assert_eq!(-1, can_complete_circuit(gas.clone(), cost.clone()));
            assert_eq!(-1, can_complete_circuit_naive(gas, cost));
        }

        #[test]
        fn example_three_starts_at_the_last_station() {
            let gas = vec![5, 1, 2, 3, 4];
            let cost = vec![4, 4, 1, 5, 1];
            assert_eq!(4, can_complete_circuit(gas.clone(), cost.clone()));
            assert_eq!(4, can_complete_circuit_naive(gas, cost));
        }

        #[test]
        fn one_station_with_enough_gas() {
            let gas = vec![1];
            let cost = vec![1];
            assert_eq!(0, can_complete_circuit(gas.clone(), cost.clone()));
            assert_eq!(0, can_complete_circuit_naive(gas, cost));
        }

        #[test]
        fn one_station_without_enough_gas() {
            let gas = vec![1];
            let cost = vec![2];
            assert_eq!(-1, can_complete_circuit(gas.clone(), cost.clone()));
            assert_eq!(-1, can_complete_circuit_naive(gas, cost));
        }

        #[test]
        fn both_algorithms_agree_on_multiple_inputs() {
            let cases = vec![
                (vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]),
                (vec![2, 3, 4], vec![3, 4, 3]),
                (vec![5, 1, 2, 3, 4], vec![4, 4, 1, 5, 1]),
                (vec![3, 3, 4], vec![3, 4, 3]),
                (vec![4, 5, 2, 6, 5, 3], vec![3, 4, 5, 1, 5, 3]),
            ];
            for (gas, cost) in cases {
                assert_eq!(
                    can_complete_circuit_naive(gas.clone(), cost.clone()),
                    can_complete_circuit(gas, cost)
                );
            }
        }
    }
}
