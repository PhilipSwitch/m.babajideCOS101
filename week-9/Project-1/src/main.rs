/* There are three sub mauin drinks
the goal of thie program is to display drinks
User inputs either Lager, Strout, otr non akcoholic
After specifyiong that he inputs his high quality drinks in the subfile
basically adds a drink to a category

*/

use std::io::Read;
use std::io::Write;
fn main() {
    let mut file = std::fs::File::create("NigerianBreweryLimited.txt").expect("CREATE FAILED");
    
    file.write_all("Lager          Stout          Non-Alcoholic\n
33 Export      Legend         Maltina\n
Desperados     Turbo King     Amstel Malta\n
Goldberg       Williams       Malta Gold\n
Gulder                        Fayrouz\n
Heineken\n
Star"
        .as_bytes())
        .expect("WRITE FAILED");
    println!("\nData written to file successfully.");
    println!();

    let mut file = std::fs::File::open("NigerianBreweryLimited.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}
