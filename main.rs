use std::io;

fn read_value<T: std::default::Default + std::str::FromStr>() -> T{
	let mut input = String::new();
	io::stdin().read_line(&mut input).unwrap();
	return input.trim().parse().unwrap_or_default();
}

fn main(){
    
}