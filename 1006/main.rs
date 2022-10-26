use std::io;

fn read_value<T: std::default::Default + std::str::FromStr>() -> T{
	let mut input = String::new();
	io::stdin().read_line(&mut input).unwrap();
	return input.trim().parse().unwrap_or_default();
}

fn main(){
    let a = read_value::<f64>();
    let b = read_value::<f64>();
    let c = read_value::<f64>();
	println!("MEDIA = {:.1}", (a*2.0+b*3.0+c*5.0)/10.0);
}