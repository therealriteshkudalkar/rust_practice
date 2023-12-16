fn transpose(matrix: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut vec: Vec<Vec<i32>> = vec![vec![0; matrix.len()]; matrix[0].len()];
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            vec[j][i] = matrix[i][j];
        }
    }
    return vec;
}

pub fn main22() {
    let vec = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    println!("vec: {:?}, transpose: {:?}", vec, transpose(&vec));
    let vec = vec![vec![1, 2, 3], vec![4, 5, 6]];
    println!("vec: {:?}, transpose: {:?}", vec, transpose(&vec));
}