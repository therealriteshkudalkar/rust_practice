use std::collections::{HashMap, HashSet};

fn is_valid_parenthesis(s: &Vec<char>) -> bool {
    let mut stack: Vec<char> = Vec::new();
    for ch in s {
        if stack.is_empty() {
            if *ch == '(' {
                stack.push(*ch);
            } else {
                return false;
            }
        } else {
            // peek on top of stack
            let peek = *stack.last().unwrap();
            if peek == '(' && *ch == ')' {
                stack.pop();
            } else if peek == '(' && *ch == '(' {
                stack.push(*ch);
            }
        }
    }
    return stack.is_empty();
}

fn collect_valid_parenthesis(hs: &mut HashSet<String>, index_hs: &mut HashSet<usize>, par_str: &Vec<char>,
                             curr_str: &mut Vec<char>, pos: usize) {
    if pos == par_str.len() {
        if is_valid_parenthesis(&curr_str) {
            hs.insert(curr_str.iter().collect());
        }
        return;
    }
    for i in 0..par_str.len() {
        //println!("pos: {pos}, i: {i}, ihs: {:?}, cond: {}", index_hs, index_hs.contains(&pos));
        if index_hs.contains(&i) {
            continue;
        }
        curr_str[pos] = par_str[i];
        index_hs.insert(i);
        collect_valid_parenthesis(hs, index_hs, par_str, curr_str, pos + 1);
        index_hs.remove(&i);
        curr_str[pos] = ' ';
    }
}

fn collect_valid_parenthesis_dp(map: &mut HashMap<usize, Vec<char>>, hs: &mut HashSet<String>,
                                index_hs: &mut HashSet<usize>, par_str: &Vec<char>,
                                curr_str: &mut Vec<char>, pos: usize) {
    if pos == par_str.len() {
        if is_valid_parenthesis(&curr_str) {
            hs.insert(curr_str.clone().iter().collect());
        }
        return;
    }
    for i in 0..par_str.len() {
        if index_hs.contains(&i) {
            continue;
        }
        if map.contains_key(&(pos + 1)) {
            println!("{:?}", map);
            // yes it contains so append the string and check if it's a valid string
            let mut prepending_vec = curr_str.to_vec();
            let mut appending_vec = map.get(&pos).unwrap().to_vec();
            let mut final_vec = Vec::new();
            final_vec.append(&mut prepending_vec);
            final_vec.push(par_str[i]);
            final_vec.append( &mut appending_vec);
            if is_valid_parenthesis(&final_vec) {
                hs.insert(final_vec.clone().iter().collect());
            }
            continue;
        }
        //println!("pos: {pos}, i: {i}, ihs: {:?}, cond: {}", index_hs, index_hs.contains(&pos));
        curr_str.push(par_str[i]);
        index_hs.insert(i);
        collect_valid_parenthesis_dp(map, hs, index_hs, par_str, curr_str, pos + 1);
        map.insert(pos, curr_str[i..].to_vec());
        index_hs.remove(&i);
        curr_str.pop();
    }
}

pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let par_str = String::new();
    // make this by adding brackets
    let mut par_vec: Vec<char> = Vec::new();
    for i in 0..2 * n {
        if i < n {
            par_vec.push('(');
        } else {
            par_vec.push(')');
        }
    }
    let mut cur_vec_str = Vec::new();
    let mut valid_par_set: HashSet<String> = HashSet::new();
    let mut index_hs: HashSet<usize> = HashSet::new();
    // collect_valid_parenthesis(
    //     &mut valid_par_set,
    //     &mut index_hs,
    //     &par_vec,
    //     &mut cur_vec_str,
    //     0);
    let mut map: HashMap<usize, Vec<char>> = HashMap::new();
    collect_valid_parenthesis_dp(
        &mut map,
        &mut valid_par_set,
        &mut index_hs,
        &par_vec,
        &mut cur_vec_str,
        0);
    println!("map: {:?}", map);
    return valid_par_set.into_iter().collect();
}

pub fn main19() {
    let test = vec!['(', '(', ')', ')'];
    println!("test: {:?}, is_valid_parenthesis: {}", test, is_valid_parenthesis(&test));
    let test = vec!['(', ')', '(', ')'];
    println!("test: {:?}, is_valid_parenthesis: {}", test, is_valid_parenthesis(&test));
    let test = vec!['(', '(', ')', '(', ')', ')'];
    println!("test: {:?}, is_valid_parenthesis: {}", test, is_valid_parenthesis(&test));
    let test = vec!['(', '(', '(', ')', ')', '(', ')', ')'];
    println!("test: {:?}, is_valid_parenthesis: {}", test, is_valid_parenthesis(&test));
    let test = vec!['(', ')', '(', ')'];
    println!("test: {:?}, is_valid_parenthesis: {}", test, is_valid_parenthesis(&test));

    let n = 2;
    println!("n: {n}, generate_parenthesis: {:?}", generate_parenthesis(n));
    let n = 3;
    println!("n: {n}, generate_parenthesis: {:?}", generate_parenthesis(n));
    // let n = 4;
    // println!("n: {n}, generate_parenthesis: {:?}", generate_parenthesis(n));
    // let n = 5;
    // println!("n: {n}, generate_parenthesis: {:?}", generate_parenthesis(n));
    // let n = 6;
    // println!("n: {n}, generate_parenthesis: {:?}", generate_parenthesis(n));
    // let n = 7;
    // println!("n: {n}, generate_parenthesis: {:?}", generate_parenthesis(n));
    // let n = 8;
    // println!("n: {n}, generate_parenthesis: {:?}", generate_parenthesis(n));
}