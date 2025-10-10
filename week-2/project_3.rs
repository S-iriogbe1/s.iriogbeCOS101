fn main() {
	let p:f64 = 510000.0;
	let r:f64 = 5.0;
	let t:f64 = 3.0;

	// Amount
	let a = p * (1.0 - (r / 100.0)).powf(t as f64);
	println!("Amount is {}", a);

	// Depreciation 
	let d = a - p;
	println!("Depreciation after three years is {}", d);
}