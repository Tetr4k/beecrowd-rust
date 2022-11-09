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
	let mut input = read_input();
	let a = input[0].parse::<i128>().unwrap_or_default();
	let b = input[1].parse::<i128>().unwrap_or_default();
	let c = input[2].parse::<i128>().unwrap_or_default();
	let d = input[3].parse::<i128>().unwrap_or_default();
    println!("{}", 
		if (b>c && d>a && c+d>a+b && c>0 && d>0 && a%2==0) {"Valores aceitos"}
		else {"Valores nao aceitos"}
	)
}