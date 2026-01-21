//Rust program to order food online from a restaurant

use std::io;

//Displaying the menu

fn main() {
    println!("\n---Welcome---");
    println!("\n   Our Menu   ");
    println!("\nCode|          Food           | Price(in Naira)");
    println!("\n P  |Poundo Yam/Edinkaiko Soup|  3200          ");
    println!("\n F  |Fried Rice & Chicken     |  3000          ");
    println!("\n A  |Amala & Ewedu Soup       |  2500          ");
    println!("\n E  |Eba & Egusi Soup         |  2000          ");           
    println!("\n W  |White Rice & Stew        |  2500          "); 

    let mut total:f32 = 0.0;   
   
    loop{

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut choice = String::new();
    let mut subtotal:f32 = 0.0;
    
    //Asking the customer what and how much theywould want

    println!("Hii,please enter the code of your order: ");
    io::stdin().read_line(&mut input1).expect("Not a valid input");
    let code = input1.trim().to_uppercase();

    println!("How many would you like: ");
    io::stdin().read_line(&mut input2).expect("Not a valid input");
    let quantity:f32 = input2.trim().parse().expect("Not a valid integer");

    //Calculating the total and subtotal costs of the customer
    if code == "P"{
        subtotal = 3200.0 * quantity;
        total = total + subtotal;
        println!("That will be {} Naira", subtotal);
    }
    else if code == "F"{
        subtotal = 3000.0 * quantity;
        total = total + subtotal;
        println!("That will be {} Naira", subtotal);
    }
    else if code == "A"{
        subtotal = 2500.0 * quantity;
        total = total + subtotal;
        println!("That will be {} Naira", subtotal);
    }
    else if code == "E"{
        subtotal = 2000.0 * quantity;
        total = total + subtotal;
        println!("That will be {} Naira", subtotal);
    }
    else if code == "W"{
        subtotal = 2500.0 * quantity;
        total = total + subtotal;
        println!("That will be {} Naira", subtotal);
    }
    else  {
        ("Type the correct code please.");
    }
//Allowing the customer to place another order if they want to
    println!("Would you like to make another order(Y/N): ");
    io::stdin().read_line(&mut choice).expect("Not a valid input");
    let mut choice = choice.trim().to_uppercase();

    if choice == "Y"{
        println!("Hello once again. Please place your next order, Thank you.");
    continue;
    }
    else if choice == "N" && total<= 10000.0{
        println!("Your total is {} Naira", total);
        println!("Thank you for checking us out. Have a lovely day");
    break;
    }
    else if choice == "N" && total > 10000.0{
        let total_disc:f32 = total*0.95;    
        println!("Your total is {} Naira", total_disc);
        println!("Thank you for checking us out. Have a lovely day");
    break;
    }
    else {
        println!("Your plans will never work");
        break;
    }
    }
}
