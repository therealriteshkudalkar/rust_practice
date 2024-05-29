// Takes O(n^2) time
fn can_complete_circuit_naive(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    let n = gas.len();
    for i in 0..n {
        // use this index as a starting position
        let mut count = 0;
        let mut curr_index = i;
        let mut fuel = 0;
        while count < n {
            fuel += gas[curr_index];
            if cost[curr_index] > fuel {
                break;
            }
            fuel -= cost[curr_index];
            curr_index = (curr_index + 1) % n;
            count += 1;
        }
        if count == n {
            return i as i32;
        }
    }
    return -1;
}

fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    // we can do some preprocessing and check from what index it is possible to move to the next
    // then from those possible indices we check if we can complete the circle
    let n = gas.len();
    let mut tank = 0;
    let mut total = 0;
    // start indicates the starting index from which it is possible to complete the circle
    // start can only be those indices from which it is possible to move to the next node
    let mut start: usize = 0;
    for i in 0..n {
        tank += gas[i] - cost[i];
        if tank < 0 {
            start = i + 1;
            total += tank;
            tank = 0;
        }
    }
    // the solution is only possible if net total amount of gas around the circle is greater than
    // or equal to zero
    return if total + tank < 0 {
        -1
    } else {
        start as i32
    };
}

pub fn main40() {
    let gas = vec![1, 2, 3, 4, 5];
    let cost = vec![3, 4, 5, 1, 2];
    println!("gas: {:?}; cost: {:?}; can_complete_circuit: {}",
             gas, cost, can_complete_circuit_naive(gas.clone(), cost.clone()));

    let gas = vec![2, 3, 4];
    let cost = vec![3, 4, 3];
    println!("gas: {:?}; cost: {:?}; can_complete_circuit: {}",
             gas, cost, can_complete_circuit_naive(gas.clone(), cost.clone()));

    let gas = vec![5, 1, 2, 3, 4];
    let cost = vec![4, 4, 1, 5, 1];
    println!("gas: {:?}; cost: {:?}; can_complete_circuit: {}",
             gas, cost, can_complete_circuit_naive(gas.clone(), cost.clone()));

    // Optimized cases
    println!("Optimized Cases");
    let gas = vec![1, 2, 3, 4, 5];
    let cost = vec![3, 4, 5, 1, 2];
    println!("gas: {:?}; cost: {:?}; can_complete_circuit: {}",
             gas, cost, can_complete_circuit(gas.clone(), cost.clone()));

    let gas = vec![2, 3, 4];
    let cost = vec![3, 4, 3];
    println!("gas: {:?}; cost: {:?}; can_complete_circuit: {}",
             gas, cost, can_complete_circuit(gas.clone(), cost.clone()));

    let gas = vec![5, 1, 2, 3, 4];
    let cost = vec![4, 4, 1, 5, 1];
    println!("gas: {:?}; cost: {:?}; can_complete_circuit: {}",
             gas, cost, can_complete_circuit(gas.clone(), cost.clone()));
}