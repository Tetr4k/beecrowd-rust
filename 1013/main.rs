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

fn maior(a: i128, b: i128) -> i128{
	(a+b+(a-b).abs())/2
}

fn main(){
   let input = read_input();
   let a = input[0].parse::<i128>().unwrap_or_default();
   let b = input[1].parse::<i128>().unwrap_or_default();
   let c = input[2].parse::<i128>().unwrap_or_default();
   println!("{} eh o maior", maior(maior(a,b),c))
}