use std::io;

fn main(){
    let mut input = String::new();
    let mut input2 = String::new();
    
    io::stdin().read_line(&mut input).unwrap();
    io::stdin().read_line(&mut input2).unwrap();

    let a: i128 = input.trim().parse::<i128>().unwrap_or_default();
    let b: i128 = input2.trim().parse::<i128>().unwrap_or_default();

    println!("X = {}", a + b);
}