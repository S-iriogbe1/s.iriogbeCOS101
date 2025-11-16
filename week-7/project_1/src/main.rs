//program to calculate the area and volume of different shapes 

use std::io;
//passing the function with parameters
fn trapezium(){
    println!("enter height");
    let mut h = String::new();
    io::stdin()
    .read_line(&mut h)
    .expect("not a valid shape");
    let h:f32 = h
    .trim()
    .parse()
    .expect("not a valid string");
    println!("enter first base");
    let mut b1 = String::new();
    io::stdin()
    .read_line(&mut b1)
    .expect("not a valid shape");
    let b1:f32 = b1
    .trim()
    .parse()
    .expect("not a valid string");
    println!("enter second base");
    let mut b2 = String::new();
    io::stdin()
    .read_line(&mut b2)
    .expect("not a valid shape");
    let b2:f32 = b2
    .trim()
    .parse()
    .expect("not a valid string");

    let trap = (h / 2.0) * (b1 + b2);
    println!("your trapezium answer is {}",trap);
}

fn rhombus(){
    let mut d1 = String::new();
    println!("enter diagonal 1");
    io::stdin()
    .read_line(&mut d1)
    .expect("not a valid shape");
    let d1:f32 = d1
    .trim()
    .parse()
    .expect("not a valid string");
    println!("enter diagonal 2");
    let mut d2 = String::new();
     io::stdin()
    .read_line(&mut d2)
    .expect("not a valid shape");
    let d2:f32 = d2
    .trim()
    .parse()
    .expect("not a valid string");
    let rhomb = (1.0/2.0) * d1 * d2;
    println!("your Rhombus answer is {}",rhomb);
}


fn parallelogram(){
    println!("enter base");
    let mut base = String::new();
     io::stdin()
    .read_line(&mut base)
    .expect("not a valid shape");
    let base:f32 = base
    .trim()
    .parse()
    .expect("not a valid string");
    println!("enter altitude");
    let mut altitude = String::new();
     io::stdin()
    .read_line(&mut altitude)
    .expect("not a valid shape");
    let altitude:f32 = altitude
    .trim()
    .parse()
    .expect("not a valid string");
    let parallel = base * altitude;
    println!("your parallelogram answer is {}",parallel);
}

fn cube(){
    let mut length = String::new();
    println!("enter your length");
     io::stdin()
    .read_line(&mut length)
    .expect("not a valid shape");
    let length:f32 = length
    .trim()
    .parse()
    .expect("not a valid string");
    let cub = 6.0 * length.powf(2.0);
    println!("your Cube answer is {}",cub);
}

fn cylinder(){
    println!("enter radius");
     let mut radius = String::new();
      io::stdin()
    .read_line(&mut radius)
    .expect("not a valid shape");
    let radius:f32 = radius
    .trim()
    .parse()
    .expect("not a valid string");
    println!("enter height");
    let mut height = String::new();
     io::stdin()
    .read_line(&mut height)
    .expect("not a valid shape");
    let height:f32 = height
    .trim()
    .parse()
    .expect("not a valid string");
    let cyl = 3.142 * radius.powf(2.0) * height;
    println!("your trapezium answer is {:2}",cyl);
}

//user inputing their choice
//CALCULATOR
fn main() {
    println!("These are the shapes this calculator can handle 
              \n 1. Trapezium
              \n 2. Rhombus
              \n 3. Parralelogram
              \n 4. Cube
              \n 5. Cylinder.");

    // user input
    println!("enter the shape you want to calculate (1-5)  ");
    let mut input1 = String::new();
    io::stdin()
    .read_line(&mut input1)
    .expect("not a valid shape");
    let input1:u32 = input1
    .trim()
    .parse()
    .expect("not a valid string");

    
   
    if input1 == 1 {
        trapezium();
    }
    else if input1 == 2{
       rhombus()
    }
    else if input1 == 3{
       parallelogram()
    }
    else if input1 == 4{
       cube()
    }
    else if input1 == 5{
       cylinder()
    }
     else if input1 >=6{
       println!("enter a number attached to the shape ")
    }

   
}
