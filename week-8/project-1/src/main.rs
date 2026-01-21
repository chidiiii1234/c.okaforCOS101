fn main() {
    // Vectors to store data as described in the table 
    let roles = vec!["Office Administrator", "Academic", "Lawyer", "Teacher"];
    
    // Example data based on the prompt's scenario 
    let staff_role = "Lawyer"; 
    let years_of_experience = 6;

    println!("Staff Role: {}", staff_role);
    println!("Years of Experience: {}", years_of_experience);

    // Logic based on the APS criteria provided 
    if staff_role == roles[2] { // Lawyer
        if years_of_experience >= 1 && years_of_experience <= 2 {
            println!("Position: APS 1-2 (Paralegal)");
        } else if years_of_experience >= 3 && years_of_experience <= 5 {
            println!("Position: APS 3-5 (Junior Associate)");
        } else if years_of_experience >= 5 && years_of_experience <= 8 {
            println!("Position: APS 5-8 (Associate)");
        } else if years_of_experience >= 8 && years_of_experience <= 10 {
            println!("Position: EL1 8-10 (Senior Associate 1-2)");
        } else if years_of_experience >= 10 && years_of_experience <= 13 {
            println!("Position: EL2 10-13 (Senior Associate 3-4)");
        } else {
            println!("Position: SES (Partner)");
        }
    }
}