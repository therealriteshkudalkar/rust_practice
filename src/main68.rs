
fn initial_state(dfa_state: &mut i32, ch: &char) {
    if *ch == '+' || *ch == '-' {
        *dfa_state = 1;
    } else if *ch == '.' {
        *dfa_state = 8;
    } else if ch.is_ascii_digit() {
        *dfa_state = 2;
    } else {
        *dfa_state = -1;
    }
}

fn first_state(dfa_state: &mut i32, ch: &char) {
    if ch.is_ascii_digit() {
        *dfa_state = 2;
    } else if *ch == '.' {
        *dfa_state = 8;
    } else {
        *dfa_state = -1;
    }
}

fn second_state(dfa_state: &mut i32, ch: &char) {
    if ch.is_ascii_digit() {
        *dfa_state = 2;
    } else if *ch == '.' {
        *dfa_state = 3;
    } else if *ch == 'e' || *ch == 'E' {
        *dfa_state = 5;
    } else {
        *dfa_state = -1;
    }
}

fn third_state(dfa_state: &mut i32, ch: &char) {
    if ch.is_ascii_digit() {
        *dfa_state = 4;
    } else if *ch == 'e' || *ch == 'E' {
        *dfa_state = 5;
    } else {
        *dfa_state = -1;
    }
}

fn fourth_state(dfa_state: &mut i32, ch: &char) {
    if ch.is_ascii_digit() {
        *dfa_state = 4;
    } else if *ch == 'e' || *ch == 'E' {
        *dfa_state = 5;
    } else {
        *dfa_state = -1;
    }
}

fn fifth_state(dfa_state: &mut i32, ch: &char) {
    if ch.is_ascii_digit() {
        *dfa_state = 7;
    } else if *ch == '+' || *ch == '-' {
        *dfa_state = 6;
    } else {
        *dfa_state = -1;
    }
}

fn sixth_state(dfa_state: &mut i32, ch: &char) {
    if ch.is_ascii_digit() {
        *dfa_state = 7;
    } else {
        *dfa_state = -1;
    }
}

fn seventh_state(dfa_state: &mut i32, ch: &char) {
    if ch.is_ascii_digit() {
        *dfa_state = 7;
    } else {
        *dfa_state = -1;
    }
}

fn eighth_state(dfa_state: &mut i32, ch: &char) {
    if ch.is_ascii_digit() {
        *dfa_state = 4;
    } else {
        *dfa_state = -1;
    }
}

fn dead_state(dfa_state: &mut i32, _: &char) {
    *dfa_state = -1;
}

pub fn is_number(s: String) -> bool {
    let mut dfa_state: i32 = 0;
    for ch in s.chars() {
        match dfa_state {
            0 => { initial_state(&mut dfa_state, &ch) }
            1 => { first_state(&mut dfa_state, &ch) }
            2 => { second_state(&mut dfa_state, &ch) }
            3 => { third_state(&mut dfa_state, &ch) }
            4 => { fourth_state(&mut dfa_state, &ch) }
            5 => { fifth_state(&mut dfa_state, &ch) }
            6 => { sixth_state(&mut dfa_state, &ch) }
            7 => { seventh_state(&mut dfa_state, &ch) }
            8 => { eighth_state(&mut dfa_state, &ch) }
            _ => { dead_state(&mut dfa_state, &ch); }
        }
    }
    dfa_state == 2 || dfa_state == 3 || dfa_state == 4 || dfa_state == 7
}
