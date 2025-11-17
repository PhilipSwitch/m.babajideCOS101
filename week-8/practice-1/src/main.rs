
fn main() {
    //Using Vec::new()
    let v : Vec<i64> = Vec::new();
    //Printing the size of vector
    println!("\nThe length of the Vec::new is: {}",v.len());

    //Using macro 
    let v = vec!["Grace","Effiong","Basil","Kareem", "Susan"];

    //pinting the size of vector
    println!("\nThe lengtg of vec macro is: {}",v.len());
}
