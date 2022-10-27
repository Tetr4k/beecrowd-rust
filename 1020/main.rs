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
    let mut tempo = read_value::<i128>();
	let anos = (tempo-tempo%365)/365;
	tempo %= 365;
	let meses = (tempo-tempo%30)/30;
	tempo %= 30;
	println!("{} ano(s)\n{} mes(es)\n{} dia(s)", anos, meses, tempo);
}