fn main() {
    use std::io::Write;

    let announce = "Nigeria Breweries Limited \n";

    let mut file = std::fs::File::create("data.txt").expect("create failed");
    file.write_all(announce.as_bytes()).expect("write failed");
    file.write_all("\nLager      | Stout     | Non-Alcoholic".as_bytes()).expect("write failed");
    file.write_all("\n-----------|-----------|--------------".as_bytes()).expect("write failed");               
    file.write_all("\n33 Export  | Legend    | Maltina      ".as_bytes()).expect("write failed"); 
    file.write_all("\nDesperados | Turbo King| Amstel Malta ".as_bytes()).expect("write failed");               
    file.write_all("\nGoldberg   | Williams  | Malta Gold   ".as_bytes()).expect("write failed");                
    file.write_all("\nGulder     |           | Fayrouz      ".as_bytes()).expect("write failed");               
    file.write_all("\nHeineken   |           |              ".as_bytes()).expect("write failed");                        
    file.write_all("\nStar       |           |              ".as_bytes()).expect("write failed");                
    println!("\nData written to file.");
}
