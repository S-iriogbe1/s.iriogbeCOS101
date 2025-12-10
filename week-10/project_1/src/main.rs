struct Laptops{
    brand:String,
    price:u32
}

fn main(){
    println!("------Assuming you are purchasing 3 from each brand ---------- ");

    let brand1 = Laptops{
        brand:String::from("HP"),
        price:650_000
    };

    let brand2 = Laptops{
        brand:String::from("IBM"),
        price:755_000
    };

    let brand3 = Laptops{
        brand:String::from("TOSHIBA"),
        price:550_000
    };

    let brand4 = Laptops{
        brand:String::from("DELL"),
        price:850_000
    };

    let total_price = brand1.price * 3 + brand2.price * 3 + brand3.price * 3 + brand4.price * 3 ;
    println!("total cost for 3 of each laptop is {}", total_price);
}