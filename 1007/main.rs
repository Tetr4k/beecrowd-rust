use std::io;

fn read_value<T: std::default::Default + std::str::FromStr>() -> T{
	let mut input = String::new();
	io::stdin().read_line(&mut input).unwrap();
	return input.trim().parse().unwrap_or_default();
}

fn main(){
	let a = read_value::<i128>();
	let b = read_value::<i128>();
	let c = read_value::<i128>();
	let d = read_value::<i128>();
    println!("DIFERENCA = {}", a * b - c * d)
}