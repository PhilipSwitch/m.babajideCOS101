//This program opens a file and read sits contents. That is displays its contents on the screen.

use std::io::Read;

fn main() {
    let mut file = std::fs::File::open("Welcome_message.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}
