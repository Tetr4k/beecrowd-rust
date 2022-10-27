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
    let mut valor = read_value::<i128>();
	println!("{}", valor);
	println!("{} nota(s) de R$ 100,00", (valor-valor%100)/100);
	valor=valor%100;
	println!("{} nota(s) de R$ 50,00", (valor-valor%50)/50);
	valor=valor%50;
	println!("{} nota(s) de R$ 20,00", (valor-valor%20)/20);
	valor=valor%20;
	println!("{} nota(s) de R$ 10,00", (valor-valor%10)/10);
	valor=valor%10;
	println!("{} nota(s) de R$ 5,00", (valor-valor%5)/5);
	valor=valor%5;
	println!("{} nota(s) de R$ 2,00", (valor-valor%2)/2);
	valor=valor%2;
	println!("{} nota(s) de R$ 1,00", valor);
}