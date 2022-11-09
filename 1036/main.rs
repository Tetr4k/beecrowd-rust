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
	let a = input[0].parse::<f64>().unwrap_or_default();
	let b = input[1].parse::<f64>().unwrap_or_default();
	let c = input[2].parse::<f64>().unwrap_or_default();
	if a!=0.0 && b*b-4.0*a*c>0.0 {
		let r1 = (-b+(b*b-4.0*a*c).sqrt())/(2.0*a);
		let r2 = (-b-(b*b-4.0*a*c).sqrt())/(2.0*a);
		println!("R1 = {:.5}\nR2 = {:.5}", r1, r2);
	}
	else{
		println!("Impossivel calcular");
	}
}