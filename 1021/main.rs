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
    let mut valor = (read_value::<f64>()*100.0) as i128;
	println!("NOTAS:");
	println!("{} nota(s) de R$ 100.00", (valor-valor%10000)/10000);
	valor=valor%10000;
	println!("{} nota(s) de R$ 50.00", (valor-valor%5000)/5000);
	valor=valor%5000;
	println!("{} nota(s) de R$ 20.00", (valor-valor%2000)/2000);
	valor=valor%2000;
	println!("{} nota(s) de R$ 10.00", (valor-valor%1000)/1000);
	valor=valor%1000;
	println!("{} nota(s) de R$ 5.00", (valor-valor%500)/500);
	valor=valor%500;
	println!("{} nota(s) de R$ 2.00", (valor-valor%200)/200);
	valor=valor%200;
	println!("MOEDAS:");
	println!("{} moeda(s) de R$ 1.00", (valor-valor%100)/100);
	valor=valor%100;
	println!("{} moeda(s) de R$ 0.50", (valor-valor%50)/50);
	valor=valor%50;
	println!("{} moeda(s) de R$ 0.25", (valor-valor%25)/25);
	valor=valor%25;
	println!("{} moeda(s) de R$ 0.10", (valor-valor%10)/10);
	valor=valor%10;
	println!("{} moeda(s) de R$ 0.05", (valor-valor%5)/5);
	valor=valor%5;
	println!("{} moeda(s) de R$ 0.01", valor);
}