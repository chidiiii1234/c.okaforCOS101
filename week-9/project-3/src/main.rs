use std::io::Write;

fn main() {
    // Data from slides [cite: 83-117]
    let data = vec![
        ("1", "Aigbogun Alamba Daudu", "South West", "Internal Affairs"),
        ("2", "Murtala Afeez Bendu", "North East", "Justice"),
        ("3", "Okorocha Calistus Ogbona", "South South", "Defense"),
        ("4", "Adewale Jimoh Akanbi", "South West", "Power & Steel"),
        ("5", "Osazuwa Faith Etieye", "South East", "Petroleum"),
    ];

    let mut file = std::fs::File::create("commissioners.txt").expect("create failed"); [cite: 17, 38]
    
    file.write_all("S/N | NAME OF COMMISSIONER | GEOPOLITICAL ZONE | MINISTRY\n".as_bytes()).expect("write failed"); [cite: 30, 38]
    file.write_all("------------------------------------------------------------------\n".as_bytes()).expect("write failed"); [cite: 30, 38]

    for (sn, name, zone, ministry) in data {
        let row = format!("{} | {} | {} | {}\n", sn, name, zone, ministry);
        file.write_all(row.as_bytes()).expect("write failed"); [cite: 30, 31, 38]
    }

    println!("Commissioner records saved to commissioners.txt");
}