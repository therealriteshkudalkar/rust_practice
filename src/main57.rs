use std::cmp::min;

pub fn trap(height: Vec<i32>) -> i32 {
    let n = height.len();

    // Find max height's index
    let mut max_height_index = 0;
    let mut max_height = height[0];
    for i in 0..n {
        if height[i] > max_height {
            max_height = height[i];
            max_height_index = i;
        }
    }

    let mut sum = 0;
    let mut lb = height[0];
    let mut initial_flag = true;
    let mut stash: Vec<i32> = Vec::new();
    for i in 0..=max_height_index {
        let curr_pillar = height[i];
        if initial_flag {
            if curr_pillar == 0 {
                continue;
            }
            lb = curr_pillar;
            initial_flag = false;
            continue;
        }
        if curr_pillar >= lb {
            sum += min(curr_pillar, lb) * stash.len() as i32;
            for num in &stash {
                sum -= *num;
            }
            lb = curr_pillar;
            stash.clear();
        } else {
            stash.push(curr_pillar);
        }
    }

    lb = height[n - 1];
    initial_flag = true;
    for i in (max_height_index..n).rev() {
        let curr_pillar = height[i];
        if initial_flag {
            if curr_pillar == 0 {
                continue;
            }
            lb = curr_pillar;
            initial_flag = false;
            continue;
        }
        if curr_pillar >= lb {
            sum += min(curr_pillar, lb) * stash.len() as i32;
            for num in &stash {
                sum -= *num;
            }
            lb = curr_pillar;
            stash.clear();
        } else {
            stash.push(curr_pillar);
        }
    }
    return sum;
}
