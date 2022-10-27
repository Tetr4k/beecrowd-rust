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
    let input = read_input();
	let x1 = input[0].parse::<f64>().unwrap_or_default();
	let y1 = input[1].parse::<f64>().unwrap_or_default();
    let input = read_input();
	let x2 = input[0].parse::<f64>().unwrap_or_default();
	let y2 = input[1].parse::<f64>().unwrap_or_default();
	println!("{:.4}", ((x2-x1).powf(2.0)+(y2-y1).powf(2.0)).sqrt());
}