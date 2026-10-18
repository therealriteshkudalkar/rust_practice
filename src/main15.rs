#[allow(dead_code)]
fn max_profit_naive(prices: &Vec<i32>) -> i32 {
    let mut max = 0;
    for i in 0..prices.len() {
        for j in (i + 1)..prices.len() {
            let tally = prices[j] - prices[i];
            if tally > max {
                max = tally;
            }
        }
    }
    max
}

fn max_profit(prices: &Vec<i32>) -> i32 {
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
    max_prof
}
