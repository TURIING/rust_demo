fn main(){
    let mut j = 1;
    for i in 1..10 {
        while j <= i {
            print!("{}x{}={:<2} ", i, j, i*j);
            j += 1;
        }
        print!("\n");
        j = 1;
    }
}