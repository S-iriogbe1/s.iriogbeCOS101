use std::io;
fn main() {
	loop{
		let mut food_type = String::new();
		let mut food_quantity = String::new();

		println!("
			MENU                         PRICE
			P = Poundo Yam/Edikiako Soup  -N3,200
			F = Fried Rice & Chicken      -N3,000
			A = Amala & Ewedu Soup        -N2,500
			E = Eba & Egusi Soup          -N2,000
			W = White Rice & Stew         -N2,500
			Enter an alphabet to make your choice: ");
		io::stdin().read_line(&mut food_type).expect("not a valid string");
		let food_type = food_type.trim().chars().next();

		println!("How many portions would you like? ( 6 portions at max)");
		io::stdin().read_line(&mut food_quantity).expect("not a valid string");
		let food_quantity:f32 = food_quantity.trim().parse().expect("invalid number");


		if food_type == Some('P') {
			let price:f32 = 3200.0;
			let total_cost = food_quantity * price;
			if food_quantity <= 6.0 {
				if total_cost > 10000.0 {
					let new_cost:f32 = total_cost - ((5.0/100.0) * total_cost);
					println!("since your order is greater than 10000 you get a discount of 5%, your total cost now is: {}", new_cost);
                    break;
				}else if total_cost < 10000.0 {
					println!("your total cost is = {}", total_cost);
			     break;
				}
			} else if food_quantity > 6.0 {
				println!("You have exceeded the portions limit");
				break;
			}
		}else if food_type == Some('F') {
			let price:f32 = 3000.0;
			let total_cost = food_quantity * price;
            if food_quantity <= 6.0 {
            	if total_cost > 10000.0 {
            		let new_cost:f32 = total_cost - ((5.0/100.0) * total_cost);
            		println!("since your order is greater than 10000 you get a discount of 5%, your total cost now is: {}", new_cost);
            		break;
            	}else if total_cost < 10000.0 {
            		println!("your total cost is = {}", total_cost);
			     break;
            	}
            }else if food_quantity > 6.0 {
            	println!("You have exceeded the portions limit");
				break;
            }
		}else if food_type == Some('A') {
			let price:f32 = 3000.0;
			let total_cost = food_quantity * price;
			if food_quantity <= 6.0 {
				if total_cost > 10000.0 {
					let new_cost:f32 = total_cost - ((5.0/100.0) * total_cost);
            		println!("since your order is greater than 10000 you get a discount of 5%, your total cost now is: {}", new_cost);
            		break;
				}else if total_cost < 10000.0 {
					println!("your total cost is = {}", total_cost);
			     break;
				}
			}else if food_quantity > 6.0 {
				println!("You have exceeded the portions limit");
				break;
			}
		}else if food_type == Some('E') {
			let price:f32 = 3000.0;
			let total_cost = food_quantity * price;
			if food_quantity <= 6.0 {
				if total_cost > 10000.0 {
					let new_cost:f32 = total_cost - ((5.0/100.0) * total_cost);
            		println!("since your order is greater than 10000 you get a discount of 5%, your total cost now is: {}", new_cost);
            		break;
				}else if total_cost < 10000.0 {
					println!("your total cost is = {}", total_cost);
			     break;
			 }
			}else if food_quantity > 6.0 {
				println!("You have exceeded the portions limit");
				break;
			}
		}else if food_type == Some('W') {
			let price:f32 = 3000.0;
			let total_cost = food_quantity * price;
			if food_quantity <= 6.0 {
				if total_cost > 10000.0 {
					let new_cost:f32 = total_cost - ((5.0/100.0) * total_cost);
            		println!("since your order is greater than 10000 you get a discount of 5%, your total cost now is: {}", new_cost);
            		break;
				}else if total_cost < 10000.0 {
					println!("your total cost is = {}", total_cost);
			     break;
				}
			}else if food_quantity > 6.0 {
				println!("You have exceeded the portions limit");
				break;
			}
		}
	}
}