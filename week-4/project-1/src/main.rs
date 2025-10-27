// Rust program to find the roots of a quadratic equation

use std::io;

fn main() {

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();
    let mut input5 = String::new();
   
    println!("Enter a: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter b: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f32 = input2.trim().parse().expect("Not a valid number");

    println!("Enter c: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f32 = input3.trim().parse().expect("Not a valid number");

    println!("You are looking for the nature of the roots,true and false: ");
    io::stdin().read_line(&mut input4).expect("Not a valid boolean");
    let nature_of_root:bool = input4.trim().parse().expect("Failed to read input");

    println!("You are looking for the roots of the equation,true or false: ");
    io::stdin().read_line(&mut input5).expect("Not a valid boolean");
    let the_roots_of_the_equation:bool = input5.trim().parse().expect("Failed to read input");

    if nature_of_root == true && the_roots_of_the_equation == false {
        let d:f32 = (b.powf(2.0)) - 4.0*a*c ;
        if d > 0.0 {
            println!("There are two distinct roots"); 
        }
        if d == 0.0 {
            println!("There is exactly one real root");
        }   
        if d < 0.0 {
            println!("There are no real roots");
        }
    }
 
    if nature_of_root == false && the_roots_of_the_equation == true {
        let x1:f32 = (((-1.0)*b) - ((b.powf(2.0))-(4.0*a*c)).powf(0.5))/(2.0*a);
        let x2:f32 = (((-1.0)*b) + ((b.powf(2.0))-(4.0*a*c)).powf(0.5))/(2.0*a);
        println!("The roots of the quadratic equation are: {} and {}", x1,x2);
    }
    
    if nature_of_root == true && the_roots_of_the_equation == true {
        let d:f32 = (b.powf(2.0)) - 4.0*a*c ;
        if d > 0.0 {
            println!("There are two distinct roots"); 
        }
        if d == 0.0 {
            println!("There is exactly one real root");
        }   
        if d < 0.0 {
            println!("There are no real roots");
        }
    }
    let x1:f32 = (((-1.0)*b) - ((b.powf(2.0))-(4.0*a*c)).powf(0.5))/(2.0*a);
    let x2:f32 = (((-1.0)*b) + ((b.powf(2.0))-(4.0*a*c)).powf(0.5))/(2.0*a);
    println!("The roots of the quadratic equation are: {} and {}", x1,x2);
}
