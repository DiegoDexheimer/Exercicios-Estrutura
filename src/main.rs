use std::vec;

use text_io::{read, scan};

fn main() {
    let line: String = read!("{}\n");
    let arr: Vec<i32> = line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    
}
