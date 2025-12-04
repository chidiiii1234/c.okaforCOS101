fn main() {
    use std::io::Write;

    let announce = "Nigeria Breweries Limited \n";

    let mut file = std::fs::File::create("data.txt").expect("create failed");
    file.write_all(announce.as_bytes()).expect("write failed");
    file.write_all("\nStudent Name   | Matric. Number | Department     | Level".as_bytes()).expect("write failed");
    file.write_all("\n---------------|----------------|----------------|------".as_bytes()).expect("write failed");               
    file.write_all("\nOluchi Mordi   | ACC10211111    | Accounting     | 300  ".as_bytes()).expect("write failed"); 
    file.write_all("\n---------------|----------------|----------------|------".as_bytes()).expect("write failed");
    file.write_all("\nAdams Aliyu    | ECO10110101    | Economics      | 100  ".as_bytes()).expect("write failed");         
    file.write_all("\n---------------|----------------|----------------|------".as_bytes()).expect("write failed");      
    file.write_all("\nShania Bolade  | CSC10328828    | Computer       | 200  ".as_bytes()).expect("write failed");         
    file.write_all("\n---------------|----------------|----------------|------".as_bytes()).expect("write failed");       
    file.write_all("\nAdekunle Gold  | EEE11020202    | Electrical     | 200  ".as_bytes()).expect("write failed");         
    file.write_all("\n---------------|----------------|----------------|------".as_bytes()).expect("write failed");      
    file.write_all("\nBlanca Edemoh  | MEE10202001    | Mechanical     | 100  ".as_bytes()).expect("write failed");            
    file.write_all("\n---------------|----------------|----------------|------".as_bytes()).expect("write failed");       
    println!("\nData written to file.");
}