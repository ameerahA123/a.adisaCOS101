fn main(){
	let t :f64 = 2.0 * 45_00000.0;
	let m :f64 = 1.0 * 15_00000.0;
	let h :f64 = 3.0 * 75_0000.0;
	let d :f64 = 3.0 * 285_0000.0;
	let a :f64 = 1.0 * 25_0000.0;

	let s = t + m + h + d + a;
    let _a = s/(2.0 + 1.0 + 3.0 + 3.0 + 1.0);
    println!("The sum is {}",s );
    println!("The average is {}",_a)
}