use text_io::read;

fn main() {
    print!("Input the number of elements to be stored in the array: ");
    let size = read!();
    let mut array = vec![0; size];

    println!("\nInput {} elements in the array: ", size);
    (0..array.len()).for_each(|i| {
        print!("element - {}: ", i);
        array[i] = read!();
    });

    array.sort();
    array.reverse();
    println!("\nElements of the array in sorted ascending order: {:?}", array);

}
