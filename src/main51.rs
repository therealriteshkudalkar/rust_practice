pub fn convert(s: String, nums_rows: i32) -> String {
    let mut converted_string = String::new();
    let vec_chars: Vec<char> = s.chars().collect();
    let total_skips = (nums_rows - 1) + (nums_rows - 2);
    for i in 0..nums_rows as usize {
        converted_string.push(vec_chars[i]);
        let mut total_skips_left = if i == 0 || i == (nums_rows - 1) as usize {
            total_skips
        } else {
            total_skips - 2
        };
        for j in (i + 1)..vec_chars.len() {
            if total_skips_left == 0 {
                converted_string.push(vec_chars[j]);
                total_skips_left = total_skips;
                continue;
            }
            total_skips_left -= 1;
        }
    }
    return converted_string;
}

pub fn main51() {
    let s = String::from("PAYPALISHIRING");
    let num_rows = 3;
    println!(
        "s: {s}; num_rows: {num_rows}; converted_string: {}",
        convert(s.clone(), num_rows)
    );
}
