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
    let tempo = read_value::<i128>();
	let horas = (tempo/3600)%60;
	let minutos = (tempo/60)%60;
	let segundos = tempo%60;
	println!("{}:{}:{}", horas, minutos, segundos)
}