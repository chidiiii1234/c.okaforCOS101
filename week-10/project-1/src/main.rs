// Define a structure for Laptop properties as taught in Slide 20
struct Laptop {
    brand: String,
    model: String,
    price: u32,
    quantity: u32,
}

// Logic to calculate total cost using a Method in Structure (Slide 23)
impl Laptop {
    fn calculate_total(&self) -> u32 {
        // Use the . operator to fetch values via the self keyword (Slide 24)
        self.price * self.quantity
    }
}

fn main() {
    // Instantiate structures for three different brands
    let hp = Laptop {
        brand: String::from("HP"),
        model: String::from("EliteBook"),
        price: 500_000,
        quantity: 10,
    };

    let dell = Laptop {
        brand: String::from("Dell"),
        model: String::from("XPS 13"),
        price: 650_000,
        quantity: 6,
    };

    let apple = Laptop {
        brand: String::from("Apple"),
        model: String::from("MacBook Pro"),
        price: 1_200_000,
        quantity: 4,
    };

    // Pass the structs to a display function (Slide 22)
    // Note: Passing them like this transfers ownership to the function
    display_laptop(hp);
    display_laptop(dell);
    display_laptop(apple);
}

// Function to fetch values and print to console (Slide 22)
fn display_laptop(l: Laptop) {
    println!("Brand: {} | Model: {}", l.brand, l.model);
    println!("Unit Price: NGN {}", l.price);
    println!("Quantity in stock: {}", l.quantity);
    println!("Total Stock Value: NGN {}\n", l.calculate_total());
}