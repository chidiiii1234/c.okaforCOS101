use std::fs;

fn main() {
    let announce = "Week 9 - Rust File Input & Output\n";
    
    let mut file = std::fs::File::create("data.txt").expect("create failed");
    fs::remove_file("data.txt").expect("could not remove file");
    println!("file is removed");
}
