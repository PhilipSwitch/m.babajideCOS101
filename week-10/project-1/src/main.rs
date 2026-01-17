// Priogram to calculate the total cost, supposing a customer purchase three products from 
// three laptop brands

// Knowledge of structs and methods must be used 
//The struct name is always equal to the impl name


use num_format::{Locale, ToFormattedString}; // Thsi si to make the final total price more raedable as it'll be separated by commas

struct Purchases {
    company:String,
    price:u64,
    number:u64,

}

//Logic to calculate the price of what user bought 
// Defining the method using impl keyword
// Methods are implemented for the struct defined
impl Purchases {
    fn prize(&self) -> u64 { // The first parameter of a structure is always self
        self.price * self.number
    }
}

fn main() {
    let brand1 = Purchases {
        company:String::from("Hewlett Packard"),
        price:650_000,
        number:3
    };
    let brand2 = Purchases {
        company:String::from("IBM"),
        price:755_000,
        number:3
    };
    let brand3 = Purchases {
        company:String::from("Toshiba"),
        price:550_000,
        number:3   
    };
    let brand4 = Purchases {
        company:String::from("Dell"),
        price:850_000,
        number:3
    };


    // Individual total price's for each brand when a customer purchases three 
    //products from each brand. 

    
    let price1 = brand1.prize(); //Total price if three products are purchased from brand1
    let price2 = brand2.prize(); //Total price if three products are purchased from brand2
    let price3 = brand3.prize(); //Total price if three products are purchased from brand3
    let price4 = brand4.prize(); //Total price if three products are purchased from brand4

    //Declaring a variable -> Total price which sums up th eindividual 
    //prices from each barans i.e. price1, price2, price3

    let total_price = price1 + price2 + price3 + price4; //Addition of all the prices to give the total price
    
    //Printing the toatl price 
    println!("");
    println!("The total price after a customer purchases three products from each brand
i.e. Hewlett Packard, IBM, Toshiba and Dell is ₦{}",total_price.to_formatted_string(&Locale::en));

    println!{"Thank you for using my program! ❤️😊"};
 
 //Company:String is unused in the program but i still put it there 


}