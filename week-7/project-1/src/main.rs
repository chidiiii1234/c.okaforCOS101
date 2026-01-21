use std::io;

// Function to handle numeric input from the user [cite: 63, 65, 66]
fn get_input(prompt: &str) -> f64 {
    println!("{}", prompt); // [cite: 64, 121]
    let mut input = String::new(); // [cite: 63, 120]
    io::stdin().read_line(&mut input).expect("Failed to read input"); // [cite: 65, 122]
    let value: f64 = input.trim().parse().expect("Please enter a valid number"); // [cite: 66, 123]
    return value; // [cite: 26, 99]
}

// 1. Area of Trapezium: height/2 * (base1 + base2) [cite: 253]
fn area_trapezium() {
    let h = get_input("Enter height:");
    let b1 = get_input("Enter base 1:");
    let b2 = get_input("Enter base 2:");
    let area = (h / 2.0) * (b1 + b2);
    println!("The Area of the Trapezium is: {}", area);
}

// 2. Area of Rhombus: 1/2 * diagonal1 * diagonal2 [cite: 254]
fn area_rhombus() {
    let d1 = get_input("Enter diagonal 1:");
    let d2 = get_input("Enter diagonal 2:");
    let area = 0.5 * d1 * d2;
    println!("The Area of the Rhombus is: {}", area);
}

// 3. Area of Parallelogram: base * altitude [cite: 255]
fn area_parallelogram() {
    let b = get_input("Enter base:");
    let a = get_input("Enter altitude:");
    let area = b * a;
    println!("The Area of the Parallelogram is: {}", area);
}

// 4. Area of Cube: 6 * (side)^2 [cite: 255]
fn area_cube() {
    let s = get_input("Enter length of the side:");
    let area = 6.0 * s.powi(2);
    println!("The Area of the Cube is: {}", area);
}

// 5. Volume of Cylinder: Pi * radius^2 * height [cite: 255]
fn volume_cylinder() {
    let r = get_input("Enter radius:");
    let h = get_input("Enter height:");
    let pi = 22.0 / 7.0; // [cite: 96, 97, 98]
    let volume = pi * r.powi(2) * h;
    println!("The Volume of the Cylinder is: {}", volume);
}

fn main() {
    println!("--- MTH 101 Calculator ---"); // [cite: 251]
    println!("Select an equation to solve:");
    println!("1. Area of Trapezium\n2. Area of Rhombus\n3. Area of Parallelogram\n4. Area of Cube\n5. Volume of Cylinder");

    let choice = get_input("Enter your choice (1-5):");

    // Logic to select the corresponding calculation [cite: 256]
    if choice == 1.0 { area_trapezium(); }
    else if choice == 2.0 { area_rhombus(); }
    else if choice == 3.0 { area_parallelogram(); }
    else if choice == 4.0 { area_cube(); }
    else if choice == 5.0 { volume_cylinder(); }
    else { println!("Invalid selection!"); }
}