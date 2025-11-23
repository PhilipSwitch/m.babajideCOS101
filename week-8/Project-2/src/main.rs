/*Program asks the administrator how many individuals are in the interview
  Program asks the administrator to input the name and experience of each person 
  The program checks the user with the highest esperience 
  The program prints out the the name of the user with the highest years of experience
*/
// PROGRAM THAT CHECK THE DEVELOPER WITH THE HIGHEST EXPERIENCE IN AN INTERVIEW
use std::io;

fn rechecker() -> String {
    let mut redo = String::new();
    std::io::stdin().read_line(&mut redo).expect("Invalid input!");
    redo.trim().to_uppercase()
}
fn restarter(x: &str) -> bool {
    println!("Kindly confirm the {} of the developers above!",x);
    println!("Kindly input Y to proceed if the {}  of each  developer is correct",x);
    println!("Kindly input N to change/edit the {} of the developers above",x);
    
    //Loop for user to recheck/proceed/edit the names of the Developers
    
    let redo = rechecker();

    
    if redo == "Y" {
        true
    }else if redo == "N" {
        false
    }else {
        println!("Invalid input. Starting again...");
        false
    }
}

fn main() {
    //create an empty vector "Names"
    let mut name : Vec<String> = Vec::new();
    // Print names vector
    //Push new elements into
    let mut input1 = String::new();
    println!("How many Developers were interviewed");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let name_num:i32 = input1.trim().parse().expect("Invalid input");
    
    loop{
        name.clear(); //Clears the vector if we are restarting this prevents data accumulation in the vector 
    for count in 0..name_num {
        let mut input2 = String::new();
        println!("Enter the name of Developer {} in the format: Firstname_Lastname", count+1);
        io::stdin().read_line(&mut input2).expect("Failed to read input");
        let new_name:String = input2.trim().parse().expect("Invalid input");
        name.push(new_name);
    }
    print!("The names of the developers are:\n");
    let mut count = 1;
    //loop to iterate elements in vector
    for i in &name {
        //iterating through i on the vector
        println!("{} {}",count, i);
        count += 1;
    }
    
    
    if restarter("names") { // Passes the string names as a parameter
        break; // breaks into the restarter function
        }
    }
      
    
    //create an empty vector "Experience"
    let mut experience : Vec<u64> = Vec::new();
    let experience_num = name_num;

    loop {
        experience.clear(); //Clears the vector if we are restarting this prevents data accumulation in the vector 
    for count in 0..experience_num {
        let mut input4 = String::new();
      //println!("Enter the experience of Developer {}", count+1);
        println!("How many years of experience does Developer {} have ?", count+1);
        io::stdin().read_line(&mut input4).expect("Failed to read input");
        let new_experience:u64 = input4.trim().parse().expect("Invalid input enter a number!");
        experience.push(new_experience);
    }
    
    for i in 0..name.len()
    {
        //iterating through i on the vector
        print!("{} has {} years experience\n",name[i],experience[i]);
    }
    
    
    if restarter("expereince") { //Passes the string names as a parameter
        break; // breaks into the restarter function
        }
    }
      

    //Loop through every number in the vector
    let mut largest = experience[0];
    for num in &experience {
        if *num > largest {
        largest = *num; //Update the largest if we find a bigger number
        }
    }

    // Print the result
    println!("The Developer(s) with the highest experience has/have an experience of {} years",largest);
    println!("Name(s) of Developer(s) with the highest experience:");

    for i in 0..experience.len() {
    if experience[i] == largest {
        println!("{}", name[i]);
    }
}
}




