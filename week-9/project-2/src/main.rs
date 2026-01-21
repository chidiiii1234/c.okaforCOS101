use std::io::Write;

fn main() {
    // Data from the provided table 
    let names = vec!["Oluchi Mordi", "Adams Aliyu", "Shania Bolade", "Adekunle Gold", "Blanca Edemoh"];
    let matrics = vec!["ACC10211111", "ECO10110101", "CSC10328828", "EEE11020202", "MEE10202001"];
    let depts = vec!["Accounting", "Economics", "Computer", "Electrical", "Mechanical"];
    let levels = vec!["300", "100", "200", "200", "100"];

    let mut file = std::fs::File::create("pau_smis.txt").expect("create failed"); [cite: 17, 38]
    
    let header = "Student Name | Matric. Number | Department | Level\n";
    let separator = "------------------------------------------------------\n";
    
    file.write_all(header.as_bytes()).expect("write failed"); [cite: 30, 38]
    file.write_all(separator.as_bytes()).expect("write failed"); [cite: 30, 38]

    for i in 0..names.len() {
        let details = format!("{} | {} | {} | {}\n", names[i], matrics[i], depts[i], levels[i]);
        print!("{}", details); // Display details [cite: 74]
        file.write_all(details.as_bytes()).expect("write failed"); [cite: 30, 31, 38]
    }

    println!("\nStudent data successfully saved to pau_smis.txt");
}