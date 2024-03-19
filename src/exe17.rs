use text_io::read;

fn main() {
    print!("Input the size of array : ");
    let size: usize = read!();
    let mut arr = vec![0; size];
    println!("\nInput {} elements in the array in ascending order:", size);
    (0..arr.len()).for_each(|i | {
        print!("element - {} : ", i);
        arr[i] = read!();
    });

    arr.sort();
    arr.dedup();
    println!("{}", arr[1]);
}
