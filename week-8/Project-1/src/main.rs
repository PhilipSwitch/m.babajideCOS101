// Program that validates staff level. Prints out the staffs level
// User enters his Job 
// Jobs are either Office administrator, Academic, Lawyer or teacher
//After user inputs his Job the program displays subjobs he chooses one then the program matches the name of
//the subjob with his Job then pprints out the works experience \
// Use vectors, tuples , knowledge of functions and also user input


// PROJECT APS LEVEL CHECKER FOR THE NIGERIAN GOVERNMENT
use std::io;
fn main() {
    loop {

    let mut input1 = String::new();
    let mut redo   = String::new();

    
    let office_administrator: Vec<&str> = vec!["Intern","Administrator","Senior Administrator","Office Manager","Director","CEO"];
    let academic: Vec<&str> =  vec!["","Research Assistant","PhD Candidate","Post-Doc Researcher","Senior lecturer","Dean"];
    let lawyer: Vec<&str>   =  vec!["Paralegal","Junior Associate","Associate","Senior Associate 1-2","Senior Associate 3-4","Partner"]; 
    let teacher: Vec<&str>  =  vec!["Placement","Classroom teacher","Senior Teacher","Leading Teacher","Deputy Principal","Principal"];
    println!("");
    println!("Choose from the list below the staff job ");
    println!("
              1. Office Administrator
              2. Academic
              3. Lawyer
              4. Teacher
            ");

    println!("Input the corresponding number to the Staff job ");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let code1:u32 = input1.trim().parse().expect("Not a valid number please input the correct number of the staff job post you want");
    if code1 == 1 {
        println!("Job titles: {:?}", office_administrator);
        aps_checker(office_administrator);

    } else if code1 == 2 {
        println!("Job titles: {:?}",academic);
        aps_checker(academic);

    }else if code1 == 3 {
        println!("Job titles: {:?}",lawyer);
        aps_checker(lawyer);

    }else if code1 == 4 {
        println!("Job titles: {:?}", teacher);
        aps_checker(teacher);
    }else {
        println!("Invalid job entered!");
    }
    println!("");
    println!("Type quit to exit or type restart to check the APS level of a staff");
    io::stdin().read_line(& mut redo).expect("Invalid input!");
    let redo = redo.trim().to_uppercase();

    if redo == "QUIT" {
        break;
    }else if redo == "RESTART" {
        continue;
    }else {
        println!("Invalid input. Starting again...");
    }
        }


}
fn aps_checker(x: Vec<&str>) {
    
    let mut input2 = String::new();
    println!("Enter the job title from the list above");
    io::stdin().read_line( &mut input2).expect("Failed to read Input");
    let checker = input2.trim().to_lowercase();

    if checker == x[0].to_lowercase() {
        println!("The staff level is APS 1-2 ");
    }else if checker == x[1].to_lowercase() {
        println!("The staff level is APS 3-5");
    }else if checker == x[2].to_lowercase() {
        println!("The staff level is APS 5-8");
    }else if checker == x[3].to_lowercase() {
        println!("The staff level is ELI 8-10");
    }else if checker == x[4].to_lowercase() {
        println!("The staff level is ELI 10-13");
    }else if checker == x[5].to_lowercase() {
        println!("The staff level is SES");
    }else {
        println!("Invlaid job entered, please enter the correct value");
    }
    
   

}
