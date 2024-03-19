use text_io::read;

fn main() {
    let mut matriz = [[0; 3]; 3];
    for i in 0..matriz.len() {
        for j in 0..matriz.len(){
            matriz[i][j] = read!();
        }
        
    }
    for i in 0..matriz.len() {
        for j in 0..matriz.len()  {
            if j < matriz.len() - 1{
                print!("{} ", matriz[i][j]);
            }else {
                println!("{}", matriz[i][j])
            }
            
        }
    }
}
