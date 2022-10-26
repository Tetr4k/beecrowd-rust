use std::io;

fn read_value() -> f64{
	let mut input = String::new();
	io::stdin().read_line(&mut input).unwrap();
	return input.trim().parse().unwrap_or_default();
}

fn main(){
   let a = read_value();
   let b = read_value();
   println!("MEDIA = {:.5}", (a*3.5+b*7.5)/11.0);
}