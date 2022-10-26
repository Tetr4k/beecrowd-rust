use std::io;

fn read_input() -> Vec<String>{
	let mut input = String::new();
	io::stdin().read_line(&mut input).unwrap();
    (&input).split_whitespace().map(|x| x.to_string()).collect::<Vec<String>>()
}

fn read_value<T: std::default::Default + std::str::FromStr>() -> T{
	let mut input = String::new();
	io::stdin().read_line(&mut input).unwrap();
	input.trim().parse().unwrap_or_default()
}

fn main(){
    let line = read_input();
	let n1 = line[1].parse::<i128>().unwrap_or_default();
	let v1 = line[2].parse::<f64>().unwrap_or_default();
	let line = read_input();
	let n2 = line[1].parse::<i128>().unwrap_or_default();
	let v2 = line[2].parse::<f64>().unwrap_or_default();
	println!("VALOR A PAGAR: R$ {:.2}", (n1 as f64)*v1+(n2 as f64)*v2);
}