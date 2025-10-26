//Program to calculate the roots of a Quadratic equation

use std::io;


fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter the value of a: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f64 = input1.trim().parse().expect("Not a valid number");

    println!("Enter the value of b: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f64 = input2.trim().parse().expect("Not a valid number");

    println!("Enter the value of c: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f64 = input3.trim().parse().expect("Not a valid number");



    let discriminant =  b.powi(2) - (4.0 * a * c);
    if discriminant > 0.0 {
        println!("There are two distinct roots");
    }
    else if discriminant == 0.0 {
        println!("There are equal roots");
    }
    else if discriminant < 0.0 {
        println!("There are no real roots");
    }
    let root_1 = (-b + (discriminant.sqrt()))/(2.0 * a);
    println!("Root one is {}", root_1);

    let root_2 = (-b - (discriminant.sqrt()))/(2.0 * a);
    println!("Root two is {}", root_2);

    println!("Therefore the roots of the quadratic equation are {} and {}", root_1, root_2);

}