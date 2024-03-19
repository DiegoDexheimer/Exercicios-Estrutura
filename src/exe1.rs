use text_io::read;

fn main() {
    let mut array: [i32;10] = [0;10];
    println!("Input 10 elements in the array :");
    
    (0..10).for_each(|i | {
        print!("element - {} : ",i);
        let number: i32 = read!();
        array[i] = number;
    });

    (0..10).for_each(|i| {
        print!("{} ", array[i]);
    });
    
}   
