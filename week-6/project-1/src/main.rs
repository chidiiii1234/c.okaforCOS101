//Rust program to order food

use std::io;

fn main() {
    println!("\n---Welcome---");
    println!("    Our Menu   ");
    println!("Code|          Food           | Price(in Naira)
               P  |Poundo Yam/Edinkaiko Soup|  3200
               F  |Fried Rice & Chicken     |  3000
               A  |Amala & Ewedu Soup       |  2500
               E  |Eba & Egusi Soup         |  2000
               W  |Whit Rice & Stew         |  2500")

    loop{

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    

    println!("Hii,please enter the code of your order: ");
    io::stdin().read_line(&mut input1).expect("Not a valid input");
    let code = input1.trim().parse().to_uppercase();

    println!("How many would you like: ");
    io::stdin().read_line(&mut input2).expect("Not a valid input");
    let quantity:i32 = input2.trim().parse().expect("Not a valid integer");

    

    }
}
