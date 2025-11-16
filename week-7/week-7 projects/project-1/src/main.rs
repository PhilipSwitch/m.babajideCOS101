use std::io;
fn main() {
loop{

    let mut input10  = String::new();
    let mut input11  = String::new(); 
    let mut input12  = String::new();
    let mut input13  = String::new();
    let mut redo     = String::new();

    println!("");
    println!("Welcome!");
    println!("This is a program that calculate the area and volume of some shapes");
    println!("Do you want to calculate area or volume?
             Plese input 'A' for Area or 'V' for volume");
    io::stdin().read_line(&mut input11).expect("Failed to read input");
    let code = input11.trim().to_uppercase();

 
    
    if code == "A" {

        loop {
            println!("");
        println!("Choose one of the following available options");
        println!("");
        println!("
            1. Area of a Trapezium
            2. Area of a Rhombus
            3. Area of a Parallelogram
            4. Area of a cube
            ");
        println!("");
        println!("Input the corresponding number to the area of the shape you would like to calculate");
        io::stdin().read_line(&mut input12).expect("Failed to read input");
        let code2:u32 = input12.trim().parse().expect("Not a valid number please input the correct number of the Area of the shape you want to calculate");
        if code2 == 1 {
            area_of_trapezium();
        }else if code2 == 2 {
            area_of_rhombus();
        }else if code2 == 3 {
            area_of_parallelogram();
        }else if code2 == 4 {
            area_of_cube();
        }else {
            println!("Invalid Input");
        }
        if (code2 >= 1) && (code2 <= 4) {
            break;
        }else {
            println!("Invalid input");
            continue;
        }
        
      }
  }


        
        if code == "V" {
        loop{
        println!("There's only one option available for now");
        println!("");
        println!("-Volume of a cylinder");
        println!("");
        println!("Do you want to continue with this calculation input 1 to continue or 2 to quit");
        io::stdin().read_line(&mut input13).expect("Failed to read input");
        let code3:u32 = input13.trim().parse().expect("Not a valid number please input the correct number i.e. 1 to continue or 2 to quit");


        if code3 == 1 {
            volume_of_cylinder();
        }else if code3 == 2 {
            break;
        }else {
            println!("");
            println!("Invalid input please input 1 to continue or 2 to exit");
        }
        
      } 
    } 
     

    println!("");
    println!("Type quit to exit, or type restart to make another calculation: ");
    io::stdin().read_line(&mut redo).expect("Failed to read input!");
    let redo = redo.trim().to_uppercase();

    if redo == "QUIT" {
        break;
    }else if redo == "RESTART" {
        continue;
    }else {
        println!("Invalid input please input. Starting again...");
    }
  }
 } 
 

fn area_of_trapezium() {

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Welcome! This is a program that calculates the area of a trapezium effortlessly");
    println!("Input the height of the trapezium in m");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let a:f64 = input1.trim().parse().expect("Invalid input");

    println!("Input the length of the first parallel side in m");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let b:f64 = input2.trim().parse().expect("Invalid input");

    println!("Input the length of the second parallel side in m");
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let c:f64 = input3.trim().parse().expect("Invalid input");
    
    let d:f64 = (a/2.0) * (b + c);
    println!("The area of the trapezium is:{}m^2", d);

}
fn area_of_rhombus() {

    let mut input4 = String::new();
    let mut input5 = String::new();
    println!("Welcome! This is a program that calculates the area of a rhombus effortlessly");
    println!("Input the length of the first diagonal in m");
    io::stdin().read_line(&mut input4).expect("Failed to read input");
    let e:f64 = input4.trim().parse().expect("Invalid input");

    println!("Input the length of the second diagonal in m");
    io::stdin().read_line(&mut input5).expect("Failed to read input");
    let f:f64 = input5.trim().parse().expect("Invalid input");
    
    let g:f64 = (1.0/2.0) * (e * f);
    println!("The area of the rhombus is: {}m^2 ", g);  
}
fn area_of_parallelogram() {
    let mut input6 = String::new();
    let mut input7 = String::new();

    println!("Welcome! This is a program that calculates the area of a parallelogram effortlessly");
    println!("Input the length of the base of the parallelogram in m");
    io::stdin().read_line(&mut input6).expect("Failed to read input");
    let h:f64 = input6.trim().parse().expect("Invalid input");

    println!("Input the length of the altitude / height of the parallelogram in m");
    io::stdin().read_line(&mut input7).expect("Failed to read input");
    let i:f64 = input7.trim().parse().expect("Invalid input");
    
    let j:f64 = h * i;
    println!("The area of the parallelogram is: {}m^2", j);
}
fn volume_of_cylinder() {
    let mut input8 = String::new();
    let mut input9 = String::new();
    println!("Welcome! This is a program that calculates the volume of a cylinder effortlessly");
    println!("Input the radius of the cylinder in m");
    io::stdin().read_line(&mut input8).expect("Failed to read input");
    let k:f64 = input8.trim().parse().expect("Invalid input");

    println!("Input the height of the cylinder in m");
    io::stdin().read_line(&mut input9).expect("Failed to read input");
    let l:f64 = input9.trim().parse().expect("Invalid input");
    let pi:f64 = 22.0/7.0 ;
    let m:f64 = pi * k.powf(2.0) * l;
    println!("The volume of the cylinder is: {}m^3", m);
}
fn area_of_cube() {
    let mut input10 = String::new();
    println!("Welcome! This is a program that calculates the area of a cube effortlessly");
    println!("Input the length of side of the cube in m");
    io::stdin().read_line(&mut input10).expect("Failed to read input");
    let n:f64 = input10.trim().parse().expect("Invalid input");
    let o:f64 = 6.0 * (n.powf(2.0));
    println!("The area of the cube is: {}m^2", o);    
}  
  
