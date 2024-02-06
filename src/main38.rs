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

pub fn main38() {
    let citations = vec![3, 0, 6, 1, 5];
    println!("citations: {:?}; h_index: {}", citations, h_index(citations.clone()));

    let citations = vec![1, 3, 1];
    println!("citations: {:?}; h_index: {}", citations, h_index(citations.clone()));

    let citations = vec![100];
    println!("citations: {:?}; h_index: {}", citations, h_index(citations.clone()));

    let citations = vec![0];
    println!("citations: {:?}; h_index: {}", citations, h_index(citations.clone()));

    let citations = vec![1];
    println!("citations: {:?}; h_index: {}", citations, h_index(citations.clone()));

    let citations = vec![7, 10, 10, 10];  // h_index = 4
    println!("citations: {:?}; h_index: {}", citations, h_index(citations.clone()));
}