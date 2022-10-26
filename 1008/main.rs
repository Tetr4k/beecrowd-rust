use std::io;

fn read_value<T: std::default::Default + std::str::FromStr>() -> T{
	let mut input = String::new();
	io::stdin().read_line(&mut input).unwrap();
	return input.trim().parse().unwrap_or_default();
}

fn main(){
    let number = read_value::<i128>();
    let hours = read_value::<i128>();
    let money = read_value::<f64>();
	println!("NUMBER = {}\nSALARY = U$ {:.2}", number, (hours as f64)*money);
}