use std::io;

fn read_value() -> i128{
	let mut input = String::new();
	io::stdin().read_line(&mut input).unwrap();
	return input.trim().parse().unwrap_or_default();
}

fn main(){
	let a = read_value();
	let b = read_value();
	println!("PROD = {}", a * b);
}