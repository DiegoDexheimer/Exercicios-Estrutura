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

    let mut unique = vec![];

    for i in 0..array.len() {
        let mut is_unique = true;
        for j in 0..array.len() {
            if i != j && array[i] == array[j] {
                is_unique = false;
                break;
            }
        }
        if is_unique {
            unique.push(array[i]);
        }
    }


    println!("\nThe unique elements found in the array are: {:?}", unique);

    

}
