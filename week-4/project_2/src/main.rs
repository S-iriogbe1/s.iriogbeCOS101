use std::io;
fn main() {
	let mut input1 = String::new();
	let mut input2 = String::new();

    println!("Enter employee experience: ");
	      io::stdin().read_line(&mut input1).expect("failed to read input");
	      let experience = input1.trim();

	println!("Enter employee age: ");
	   io::stdin().read_line(&mut input2).expect("invalid string");
	   let age:i32 = input2.trim().parse().expect("not a valid number");

    if experience == "experienced" {
		if age >= 40{
			println!("your incentive is 1560000");
		}else if age >= 30 {
			println!("your incentive is 1480000");
		}else if age >= 28 {
			println!("your incentive is 1400000");
		}else if age < 28 {
			println!("your incentive is 1300000");
		}
	}else if experience == "inexperienced" {
		println!("your incentive is 100000");
	}
}

