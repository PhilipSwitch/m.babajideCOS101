/*
Program to print and save the details of PAU SIMS in  a text file
*/

use std::io::Read;
use std::io::Write;

fn main() {
    let mut file = std::fs::File::create("SIMS.txt").expect("CREATE FAILED");
    file.write_all("Student Name          Matric Number          Department          Level\n"
        .as_bytes())
        .expect("WRITE FAILED");

    let student_name = vec!["Oluchi Mordi ", "Adams Aliyu  ", "Shania Bolade", "Adekunle Gold", "Blanca Edemoh"];
    let matric_num = vec!["ACC10211111", "ECO10110101", "CSC10328828", "EEE11020202", "MEE10202001"];
    let dept = vec![" Accounting", " Economics ", " Computer  ", " Electrical", " Mechanical"];
    let level = vec![" 300", " 100", " 200", " 200", " 100"];
    

    for i in 0..5 {
        file.write_all(format!("{}          {}          {}          {}\n", student_name[i], matric_num[i], dept[i], level[i]).as_bytes()).expect("WRITE FAILED");
    }
    println!("\nData written to file successfully.");
    println!();

    let mut file = std::fs::File::open("SIMS.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}