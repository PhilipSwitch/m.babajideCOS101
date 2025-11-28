//Program that deletes a file data.txt wheen it is created in the  practice-3 directory
// I've ran this code and it has deleted the file. So to test this program create another file data.txt to dee if it deletes 

use std::fs;


fn main() {
    fs::remove_file("data.txt").expect("could not remove file");
    println!("file is removed");
}
