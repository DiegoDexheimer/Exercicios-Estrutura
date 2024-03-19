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

    print!("Input the value to be insuse text_io::read;

    fn main() {
        print!("Input the size of array : ");
        let size: usize = read!();
        let mut arr = vec![0; size];
        println!("\nInput {} elements in the array in ascending order:", size);
        (0..arr.len()).for_each(|i | {
            print!("element - {} : ", i);
            arr[i] = read!();
        });
    
        print!("Input the position where to delete: ");
        let index: usize = read!();
        
        arr.remove(index - 1);
    
        println!("{:?}", arr);
    
    }
    erted : ");
    let value: i32 = read!();
    print!("Input the Position, where the value to be inserted :");
    let position: usize = read!();
    arr.insert(position-1, value);

    println!("\n The new list is: {:?}", arr);
}
