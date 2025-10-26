// Rust program to determine the annual incentive of an emplyee

use std::io;

fn main() {

    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("You have experience: ");
    io::stdin().read_line(&mut input1).expect("Not a valid boolean");
    let has_experience:bool = input1.trim().parse().expect("Failed to read input");

    println!("How old are you: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let age:i32 = input2.trim().parse().expect("Not a valid number");

    if has_experience == true && age >= 40{
        println!("Your incentive is 1.56 million naira");
    }

    if has_experience == true && age >= 30 && age < 40 {
        println!("Your incentive is 1.48 million naira");
    } 

    if has_experience == true && age < 28 {
        println!("Your incentive is 1.3 million naira per month");

    if has_experience == false {
        println!("Your incentive is one hundred thousand naira");
    }    
    }
}   
