fn main() {
	let t:f64 = 2.00 * 450000.00;
	let m:f64 = 1500000.00;
	let h:f64 = 3.00 * 750000.00;
	let d:f64 = 3.00 * 2850000.00;
	let a:f64 = 250000.00;

 // Sum
    let s = t + m + h + d + a;
    println!("Sum is {}", s);

   // Average
    let a = t + m + h + d + a / 5.00;
    println!("Average is {}", a);
}