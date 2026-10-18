// A person is said to have h-index of h if that person has at least h papers which have
// at least h citations

fn h_index(citations: Vec<i32>) -> i32 {
    let mut sorted_citations = citations.clone();
    sorted_citations.sort();
    let n = sorted_citations.len();
    let mut i =  n;
    let mut h_index = 0;
    while i > 0 {
        let paper_count = n as i32 - i as i32 + 1;
        if sorted_citations[i - 1] >= paper_count {
            h_index += 1;
        }
        i = i.saturating_sub(1);
    }
    return h_index;
}
