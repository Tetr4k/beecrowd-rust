use std::io;

fn main(){
    const N: f64 = 3.14159;
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let r: f64 = input.trim().parse().unwrap_or_default();
    println!("A={:.4}", N*r*r);
}