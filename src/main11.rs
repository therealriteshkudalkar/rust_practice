
// store[i] = true means that if alice is given i as n then alice will definitely win
// for store[3]
// // let i go from 2 to 1
// // // check if 3 % i == 0
// // // // if yes, then check if store[i] == true
// // // // // if yes, then it's false and check if it comes out true for other i
// // // // // if no, then return true
// // // // if no, then go to next
fn divisor_game(n: i32) -> bool {
    if n == 1 {
        return false;
    } else if n == 2 {
        return true;
    }
    let mut store = vec![false; (n + 1) as usize];
    store[1] = false;
    store[2] = true;
    for i in 3..=n {
        // alice plays for every i
        let mut res = false;
        for j in (1..i).rev() {
            // alice chooses value j
            if i % j == 0 {
                // bob will play i - j
                // if bob wins, alice will lose
                res = res || !store[(i - j) as usize];
            }
        }
        store[i as usize] = res;
    }
    //println!("store: {:?}", store);
    return store[n as usize];
}

pub fn main11() {
    let n = 4;
    println!("n: {n}; divisor_game: {}", divisor_game(n));
}