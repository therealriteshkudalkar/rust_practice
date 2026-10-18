fn justify_line(words: Vec<String>, line_len: usize, max_width: usize) -> String {
    let mut justified_line = String::new();
    let mut remaining_spaces = max_width - line_len;
    if words.len() == 1 {
        justified_line.push_str(&*words[0]);
        for _ in 0..remaining_spaces {
            justified_line.push(' ');
        }
        return justified_line
    }
    let min_extra_space = remaining_spaces / (words.len() - 1);
    for i in 0..words.len() {
        justified_line.push_str(&*words[i]);
        if i != words.len() - 1 {
            justified_line.push(' ');
            let mut temp_min_extra_space = min_extra_space;
            while temp_min_extra_space > 0 {
                justified_line.push(' ');
                remaining_spaces -= 1;
                temp_min_extra_space -= 1;
            }
            let extra_possible = remaining_spaces.saturating_sub( min_extra_space * (words.len() - i - 2)) > 0;
            if extra_possible {
                justified_line.push(' ');
                remaining_spaces -= 1;
            }
        }
    }
    return justified_line;
}

fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
    let max_width = max_width as usize;
    let mut current_line: Vec<String> = Vec::new();
    let mut current_line_len = 0;
    let mut justified_lines: Vec<String> = Vec::new();
    for i in 0..words.len() {
        let word = &words[i];
        if i == words.len() - 1 {
            // It is the last word so if
            if current_line_len == 0 {
                justified_lines.push(word.clone());
            } else {
                if current_line_len + 1 + word.len() <= max_width {
                    current_line_len += 1 + word.len();
                    current_line.push(word.clone());
                    // justify the current line and append it
                    let justified_line = justify_line(current_line.clone(), current_line_len, max_width);
                    justified_lines.push(justified_line);
                } else {
                    // justify the current line and append it
                    let justified_line = justify_line(current_line.clone(), current_line_len, max_width);
                    justified_lines.push(justified_line);
                    justified_lines.push(word.clone());
                }
            }
        } else {
            if current_line_len == 0 {
                // Append the word to current line
                current_line.push(word.clone());
                current_line_len += word.len();
            } else {
                if current_line_len + 1 + word.len() <= max_width {
                    // Append the word to current line
                    current_line_len += 1 + word.len();
                    current_line.push(word.clone());
                } else {
                    // Justify the current line and append it to the justified lines
                    // calculate the remaining space
                    // divide the remaining space among into (number of words in current line - 1)
                    let justified_line = justify_line(current_line.clone(), current_line_len, max_width);
                    justified_lines.push(justified_line);
                    current_line.clear();
                    current_line.push(word.clone());
                    current_line_len = word.len();
                }
            }
        }
    }
    // process the last line by removing extra characters
    let total_lines = justified_lines.len();
    let last_line = justified_lines[total_lines - 1].clone();
    let last_line_words: Vec<&str> = last_line.split(' ').filter(|&s| !s.is_empty()).collect();
    let mut final_last_line = String::new();
    for word in last_line_words {
        final_last_line.push_str(word);
        if final_last_line.len() < max_width {
            final_last_line.push(' ');
        }
    }
    while final_last_line.len() < max_width {
        final_last_line.push(' ');
    }
    justified_lines[total_lines - 1] = final_last_line;
    return justified_lines;
}
