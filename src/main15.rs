pub fn max_profit_naive(prices: &Vec<i32>) -> i32 {
    let mut max = 0;
    for i in 0..prices.len() {
        for j in (i + 1)..prices.len() {
            let tally = prices[j] - prices[i];
            if tally > max {
                max = tally;
            }
        }
    }
    return max;
}

pub fn max_profit(prices: &Vec<i32>) -> i32 {
    // keep track of index of the cheapest stock as of yet
    // also keep track of the maximum profit
    let mut cheapest = prices[0];
    let mut max_prof = 0;
    for i in 1..prices.len() {
        if prices[i] < cheapest {
            cheapest = prices[i];
        }
        let curr_profit = prices[i] - cheapest;
        if curr_profit > max_prof {
            max_prof = curr_profit;
        }
    }
    return max_prof;
}

pub fn main15() {
    let prices = vec![7, 1, 5, 3, 6, 4];
    println!("prices: {:?}; max_profit: {}", prices, max_profit(&prices));
    let prices = vec![7, 6, 4, 3, 1];
    println!("prices: {:?}; max_profit: {}", prices, max_profit(&prices));
    let prices = vec![8, 6, 9, 7, 8, 10, 3, 2, 4, 3, 12];
    println!("prices: {:?}; max_profit: {}", prices, max_profit(&prices));
}