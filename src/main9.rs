fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    if num_rows == 1 {
        return vec![vec![1]];
    }
    let mut pas_tri: Vec<Vec<i32>> = vec![Vec::new(); num_rows as usize];
    pas_tri[0] = vec![1];
    pas_tri[1] = vec![1, 1];
    for i in 2..num_rows {
        // build pascal's (i+1)th vector
        // length of this vector is (i+1)
        let mut pas_ith = vec![1; (i + 1) as usize];
        for j in 1..=((i + 1) / 2) {
            let computed_val =  pas_tri[(i - 1) as usize][(j - 1) as usize] + 
                pas_tri[(i - 1) as usize][j as usize];
            pas_ith[j as usize] = computed_val;
            pas_ith[(i  - j) as usize] = computed_val;
        }
        pas_tri[i as usize] = pas_ith;
    }
    pas_tri
}

pub fn main9() {
    let row = 5;
    println!("rows: {row}, generate: {:?}", generate(row));
}