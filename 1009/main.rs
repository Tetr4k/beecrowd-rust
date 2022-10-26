use std::io;

fn read_value<T: std::default::Default + std::str::FromStr>() -> T{
	let mut input = String::new();
	io::stdin().read_line(&mut input).unwrap();
	return input.trim().parse().unwrap_or_default();
}

fn main(){
    let _name = read_value::<String>();
	let salary = read_value::<f64>();
	let total = read_value::<f64>();
	println!("TOTAL = R$ {:.2}", salary+total*0.15);
}