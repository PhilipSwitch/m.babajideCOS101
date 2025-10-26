use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();


//Program to allow the user select an option


    println!("Enter your experience");
    println!("Please select an option:");
    println!("
             Experienced -   Enter 'A'
             Inexperienced - Enter 'B'
            ");
    println!("Enter your option");

    let experienced:bool = true;
    io::stdin().read_line(&mut input1).expect("Invalid input please enter the correct option");
    let choice:String = input1.trim().to_lowercase();

    if choice == "a" {
        let experienced = true;
        println!("You're ready to move on to the next step");
    }
    else if choice == "b" {
        let experienced = false;
        println!("You're ready to move on to the next step");
    }
    else{
        println!("Invalid input please input the correct option");
    }


    // matching user responses
    // match input1 {
    //  experienced         => println!("You selected option A"),
    //  inexperienced       => println!("You selected option B"),
    // }


    
//program to allow the user enter his age
     println!("Enter your age:");
     io::stdin().read_line(& mut input2).expect("Not a valid string");
     let age:u64 = input2.trim().parse().expect("Not a valid number");


//program to give user his incentive based on requirements
     if experienced == true && age >= 40 {
        println!("Your incentive is N 1,560,000 per month");
     }
     else if experienced == true && (age >= 30 && age < 40) {
        println!("Your incentive is N 1,480,000 per month");
     }
     else if experienced == true && age < 28 {
        println!("Your incentive is N 1,300,000 per month");
     } 
     else if experienced == false {
        println!("You're incentive is N 100,000");
     }


   
}