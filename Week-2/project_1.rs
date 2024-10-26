fn main(){
	let p:f64 = 52_0000000.0;
	let r:f64 = 10.0;
	let n:f64 = 5.0;

	let a = p * ( 1.0 + (r / 100.0)) * n;
	let ci = a - p;
	println!("Amount is {}", a);
	println!("Simple Interest is {}",ci);

}