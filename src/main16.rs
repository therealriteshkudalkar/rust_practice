
fn get_maximum_generated(n: i32) -> i32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }
    let mut gen_vec = vec![0; (n + 1) as usize];
    gen_vec[0] = 0;
    gen_vec[1] = 1;
    let mut max = 1;
    for i in 2..=n {
        let gen_val = if i % 2 == 0 {
            gen_vec[(i/2) as usize]
        } else {
            gen_vec[(i/2) as usize] + gen_vec[(i/2 + 1) as usize]
        };
        gen_vec[i as usize] = gen_val;
        if max < gen_val {
            max = gen_val;
        }
    }
    max
}
