fn main() {
   
   let fullname = "CHibudum John Umeh";
   let depatment = "Computer Science";
   let uni = "PAn-Atlantic University";

   let mut school = "School of Science".to_string();
   //push string
   school.push_str(" and Technology");

   println!("My name is: {}", fullname);
   // check length
   println!("The length my full name is: {}",fullname.len());
   println!("I am a student of {} Department", depatment);
   println!("{}",school);
   println!("{}",uni);  
}
