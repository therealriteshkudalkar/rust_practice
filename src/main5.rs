
fn find_index(vector: &Vec<i32>, money: i32) -> usize {
    //let index: usize = 0;
    // find the first index which I can make 8 with given money
    // if all are 8 then provide the last index
    for (index, item) in vector.iter().enumerate().rev() {
        if *item + money == 8 {
            return index;
        }
        if *item == 8 {
            return index;
        }
        if *item != 8 {
            if *item + money == 4 {
                continue;
            } else {
                return index;
            }
        }
    }
    return 0;
}

fn dist_money(money: i32, children: i32) -> i32 {
    let mut temp = money - children;
    if temp < 0 {
        return -1;
    }

    let mut children_vec = vec![1; children as usize];
    for i in 0..children_vec.len() {
        // check if 7 can be given or not
        if temp - 7 >= 0 {
            children_vec[i] += 7;
            temp -= 7;
        } else {
            // give the money and check if it equates to 4 before giving it away
            if children_vec[i] + temp == 4 {
                // under this temp and children_vec[i] are both less than 4
                if temp == 1 {
                    continue;
                } else if temp == 2 {
                    temp -= 1;
                    children_vec[i] += 1;
                } else {
                    temp -= 2;
                    children_vec[i] += 2;
                }
            } else {
                children_vec[i] += temp;
                temp = 0;
            }
        }
    }
    // check if temp is still left after giving away money
    if temp != 0 {
        // find the index where money should be given to maximize 8s in the array
        let index = find_index(&children_vec, temp);
        children_vec[index] += temp;
    }

    // check how many 8s are present in the array
    let mut count = 0;
    for item in children_vec.iter() {
        if *item == 8 {
            count += 1;
        }
    }
    return count;
}

pub fn main5() {
    let (money, children) = (20, 3);
    println!("money: {money}, children: {children}, dist_money: {}", dist_money(money, children));

    let (money, children) = (24, 3);
    println!("money: {money}, children: {children}, dist_money: {}", dist_money(money, children));

    let (money, children) = (16, 2);
    println!("money: {money}, children: {children}, dist_money: {}", dist_money(money, children));

    let (money, children) = (255, 3);
    println!("money: {money}, children: {children}, dist_money: {}", dist_money(money, children));

    let (money, children) = (28, 4);
    println!("money: {money}, children: {children}, dist_money: {}", dist_money(money, children));

    let (money, children) = (12, 2);
    println!("money: {money}, children: {children}, dist_money: {}", dist_money(money, children));
}