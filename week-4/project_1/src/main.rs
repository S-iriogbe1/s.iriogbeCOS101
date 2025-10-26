use std::io;
fn main() {
	let mut input1 = String::new();
	let mut input2 = String::new();
	let mut input3 = String::new();

	println!("Enter a: ");
	io::stdin().read_line(&mut input1).expect("invalid string");
	let a:f32 = input1.trim().parse().expect("Not a valid number");

	println!("Enter b: ");
	io::stdin().read_line(&mut input2).expect("invalid string");
	let b:f32 = input2.trim().parse().expect("Not a valid number");

	println!("Enter c: ");
	io::stdin().read_line(&mut input3).expect("invalid string");
	let c:f32 = input3.trim().parse().expect("Not a valid number");

	let discriminant:f32 = b.powf(2.0) - 4.0 * a * c;
	if discriminant > 0.0 {
	   let root1 = (-b - discriminant.sqrt()) / (2.0 * a);
	   let root2 = (-b + discriminant.sqrt()) / (2.0 * a);
	   println!("The two real roots are: {} and {}", root1, root2);

	} else {
	  println!("there are no real roots");

	}
	if discriminant == 0.0 {
		println!("there is excatly one real root");
	}


}