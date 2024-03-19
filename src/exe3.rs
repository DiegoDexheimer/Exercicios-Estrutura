use text_io::read;

fn main() {
    let mut size: usize;
    print!("Input the number of elements to be stored in the array : ");
    size = read!();

    let mut array: Vec<i32> = vec![0; size];
    println!("\nInput {} elements in the array :", size);

    (0..array.len()).for_each(|i |{
        print!("element - {} : ", i);
        array[i] = read!();
    });

    println!("Sum of all elements stored in the array is : {}", array.iter().sum::<i32>());
}
