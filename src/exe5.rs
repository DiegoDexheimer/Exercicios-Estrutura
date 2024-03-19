use text_io::read;

fn main() {
    print!("Input the number of elements to be stored in the array: ");
    let size: usize = read!();
    let mut array = vec![0; size];

    println!("\nInput {} elements in the array: ", size);
    (0..array.len()).for_each(|i| { 
        print!("element - {} : ", i);
        array[i] = read!();
    });

    array.sort();

    let mut duplicates = 0;

    (0..array.len()-1).for_each(|i| {
        if array[i] == array[i+1] {
            duplicates += 1;
        }
    });

    println!("Total number of duplicate elements found in the array is : {}", duplicates);

}
