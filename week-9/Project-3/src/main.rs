use std::io::Read;
use std::io::Write;

fn main() {
    let mut file = std::fs::File::create("EFCC.txt").expect("CREATE FAILED");
    file.write_all("Name of commisioner            Ministry               Geopolitical Zone\n"
        .as_bytes())
        .expect("WRITE FAILED");


    let com_name = vec!["Aigbogun Alamba Daudu   ", "Murtala Afeez Bendu     ", "Okorocha Calistus Ogbona", "Adewale Jimoh Akanbi    ", "Osazuwa Faith Etieye",];
    let min = vec!["Internal Affairs", "Justice         ", "Defense         ", "Power & Steel   ", "    Petroleum"];
    let geo_zone = vec!["South West ", "North East ", "South South", "South West ", "       South East",];


    for i in 0..5 {
        file.write_all(format!("{}       {}       {}\n", com_name[i], min[i], geo_zone[i]).as_bytes()).expect("WRITE FAILED");
    }
    println!("\nData written to file successfully.");
    println!();

    let mut file = std::fs::File::open("EFCC.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}