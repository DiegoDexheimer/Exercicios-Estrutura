use std::vec;

use text_io::{read, scan};

fn main() {
    let line: String = read!("{}\n");
    let arr: Vec<i32> = line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let sum: i32 = read!();

    for i in 0..arr.len() {
        for j in 0..arr.len() {
            if i != j && arr[i] + arr[j] == sum {
                println!(
                    "Pair of elements can make the given sum by the value of index {} and {}",
                    i, j
                );
                return;
            }
        }
    }
}
