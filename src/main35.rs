fn max_profit(prices: Vec<i32>) -> i32 {
    // Make an array of increasing or decreasing
    let mut increasing_array = vec![false; prices.len() - 1];
    for i in 1..prices.len() {
        increasing_array[i - 1] = prices[i] > prices[i - 1];
    }
    // Loop through the array following these rules
    // Purchase when you first notice increasing
    // Sell when you first notice decreasing
    let mut profit = 0;
    let mut holding_stock = false;
    let mut currently_held_stock_price = 0;
    for i in 0..increasing_array.len() {
        if increasing_array[i] {
            if !holding_stock {
                currently_held_stock_price = prices[i];
                holding_stock = true;
            }
            if holding_stock {
                if i == increasing_array.len() - 1 {
                    let current_stock_value = prices[i + 1];
                    profit += current_stock_value - currently_held_stock_price;
                    currently_held_stock_price = 0;
                    holding_stock = false;
                }
            }
        } else {
            if holding_stock {
                let current_stock_value = prices[i];
                profit += current_stock_value - currently_held_stock_price;
                currently_held_stock_price = 0;
                holding_stock = false;
            }
        }
    }
    return profit;
}

pub fn main35() {
    let stock_variation = vec![7,1,5,3,6,4];
    println!("stock_variation: {:?}; max_profit: {}", stock_variation,
             max_profit(stock_variation.clone()));

    let stock_variation = vec![1,2,3,4,5];
    println!("stock_variation: {:?}; max_profit: {}", stock_variation,
             max_profit(stock_variation.clone()));

    let stock_variation = vec![7,6,4,3,1];
    println!("stock_variation: {:?}; max_profit: {}", stock_variation,
             max_profit(stock_variation.clone()));
}