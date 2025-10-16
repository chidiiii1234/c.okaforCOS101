fn main() {
	let t:f64 = 2.0 * 450000.0;
	let m:f64 = 1.0 * 1500000.0;
	let h:f64 = 3.0 * 750000.0;
	let d:f64 = 3.0 * 2850000.0;
	let c:f64 = 1.0 * 250000.0;

	//sum and average
	let s:f64= t + m + h + d + c;
	println!("Sum is {}", s);
	let a:f64= s/5.0;
	println!("Average is {}", a);
}