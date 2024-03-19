use text_io::read;

fn main() {
    let size: usize;
    print!("Input the number of elements to be stored in the array :");
    size = read!();

    let mut array = vec![0; size];
    
    println!("\nInput {} elements in the array :", size);

    (0..size).for_each(|i | {
        print!("element - {} : ", i);
        array[i] = read!();
    });

    println!("The elements stored in the first array are :");
    (0..size).for_each(|i|{
        print!("{} ", array[i]);
    });

    let mut array2 = vec![0; size];

    (0..size).for_each(|i|{
        array2[i] = array[i];
    });

    println!("\nThe elements copied into the second array are :");
    (0..size).for_each(|i|{
        print!("{} ", array2[i]);
    });

}   
