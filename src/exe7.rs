use text_io::read;

fn main() {
    print!("Input the number of elements to be stored in the first array :");
    let mut size = read!();
    let mut array = vec![0; size];

    println!("\nInput {} elements in the array: ", size);
    (0..array.len()).for_each(|i| {
        print!("element - {}: ", i);
        array[i] = read!();
    });

    print!("Input the number of elements to be stored in the second array :");
    size = read!();
    let mut array2 = vec![0; size];
    println!("\nInput {} elements in the array: ", size);
    (0..array2.len()).for_each(|i| {
        print!("element - {}: ", i);
        array2[i] = read!();
    });

    (0..array.len()).for_each(|i| {
        array.push(array2[i]);
    });
    array.sort();
    array.reverse();

    println!("The merged array in decending order is :");
    (0..array.len()).for_each(|i| {
        print!("{} ", array[i]);
    });

}
