use std::vec;

use text_io::read;

fn main() {
    print!("Input the size of the square matrix (less than 5): ");
    let size: usize = read!();
    let mut matrix = vec![vec![0; size]; size];
    let mut matrix2 = vec![vec![0; size]; size];

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            print!("Input the value of the element at position ({}, {}): ", i, j);
            matrix[i][j] = read!();
        }
    }

    for i in 0..matrix2.len() {
        for j in 0..matrix2[i].len() {
            print!("Input the value of the element at position ({}, {}): ", i, j);
            matrix2[i][j] = read!();
        }
    }
    let mut sum = vec![vec![0; size]; size];
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            sum[i][j] = matrix[i][j] - matrix2[i][j];
        }
    }
    println!("{:?}", sum);
}
