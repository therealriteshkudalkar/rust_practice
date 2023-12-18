use std::collections::HashMap;

fn contains_nearby_duplicate(nums: &Vec<i32>, k: i32) -> bool {
    let mut map: HashMap<i32, Vec<usize>> = HashMap::new();
    for (index, num) in nums.iter().enumerate() {
        if map.contains_key(num) {
            map.get_mut(num).unwrap().push(index);
        } else {
            let new_vec = vec![index];
            map.insert(*num, new_vec);
        }
    }
    for (_, value) in map {
        if value.len() > 1 {
            // loop through the array pair and check if the two indices are less than k
            for i in 0..(value.len() - 1) {
                let diff = (value[i + 1] as i32 - value[i] as i32).abs();
                if diff <= k {
                    return true;
                }
            }
        }
    }
    return false;
}

pub fn main26() {
    let vec = vec![1, 2, 3, 1];
    let k = 3;
    println!("vec: {:?}, k: {k}, contains_nearby_duplicate: {}", vec, contains_nearby_duplicate(&vec, k));

    let vec = vec![1, 0, 1, 1];
    let k = 1;
    println!("vec: {:?}, k: {k}, contains_nearby_duplicate: {}", vec, contains_nearby_duplicate(&vec, k));

    let vec = vec![1, 2, 3, 1, 2, 3];
    let k = 1;
    println!("vec: {:?}, k: {k}, contains_nearby_duplicate: {}", vec, contains_nearby_duplicate(&vec, k));
}