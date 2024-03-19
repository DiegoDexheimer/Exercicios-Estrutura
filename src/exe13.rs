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

    print!("Input the value to be inserted : ");
    let number: i32 = read!();
    println!("\nThe exist array list is :");
    array.iter().for_each(|i| print!("{} ", i));
    
    array.push(number);
    array.sort();
    println!("\nAfter Insert the list is :");
    array.iter().for_each(|i| print!("{} ", i));

}
