use text_io::read;

fn main() {
    print!("Input the number of elements to store in the array :");
    let n: usize = read!();
    let mut array: Vec<i32> = vec![0; n];

    println!("Input {} number of elements in the array :", &n);
    for i in 0..n {
        print!("element - {} : ", i);
        array[i] = read!();
    }

    println!("The values store into the array are :");
    for i in 0..array.len() {
        print!("{} ", array[i]);
    }

    array.reverse();
   println!("\nThe values store into the array in reverse are :");
    for i in 0..array.len() {
        print!("{} ", array[i]);
    }
}
