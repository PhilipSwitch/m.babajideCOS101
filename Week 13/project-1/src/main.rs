use std::io::Read;

fn file_opener(x:&str) {
    let mut file = std::fs::File::open(x).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}",contents);

}

fn main() {
    loop {
        let mut input = String::new();
        println!("Enter status (administrator, project manager, employee, customer, vendor, or exit to exit the program): ");
        std::io::stdin().read_line(&mut input).expect("FAILED TO READ INPUT");
        let input = input.trim().to_lowercase();

        // Using if else if else instead of match
        if input == "administrator" || input == "admin" {
            file_opener("globacom_db.sql");
        } else if input == "project manager" {
            file_opener("project_tb.sql");
        } else if input == "employee" {
            file_opener("staff_tb.sql");
        } else if input == "customer" {
            file_opener("customer_tb.sql");
        } else if input == "vendor" {
            file_opener("dataplan_tb.sql");
        } else if input == "exit" {
            break;
        } else {
            println!("Invalid status entered.");
        }

        println!(); // Print a newline for better readability between iterations
    }
}
