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
	const PI: f64 = 3.14159;
	let input = read_input();
	let a = input[0].parse::<f64>().unwrap_or_default();
	let b = input[1].parse::<f64>().unwrap_or_default();
	let c = input[2].parse::<f64>().unwrap_or_default();
    println!(
		"TRIANGULO: {:.3}\nCIRCULO: {:.3}\nTRAPEZIO: {:.3}\nQUADRADO: {:.3}\nRETANGULO: {:.3}",
		(a*c)/2.0, PI*c*c, (a+b)*c/2.0, b*b, a*b
	)
}