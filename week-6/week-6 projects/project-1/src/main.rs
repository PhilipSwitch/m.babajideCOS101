use std::io;
fn main() {
    
loop{
    println!("");
    println!("Code      Menu                                Price(N)");
    println!("P         Poundoo Yam/Edinkaikpoo Soup          N3,200");
    println!("F         Fried Rice & Chicken                  N3,000");
    println!("A         Amala & Ewedu Soup                    N2,500");
    println!("E         Eba & Egusi soup                      N2,000");
    println!("W         White Rice & Stew                     N2,500");

    let p:u32 = 3_200;
    let f:u32 = 3_000;
    let a:u32 = 2_500;
    let e:u32 = 2_000;
    let w:u32 = 2_500;

    let mut input1 = String::new();
    println!("Enter the Code of your desired food on the menu: ");
    io::stdin().read_line(&mut input1).expect("Failed to read input!");
    let code = input1.trim().to_uppercase();

    let mut input2 = String::new();
    println!("Enter the quantity: ");
    io::stdin().read_line(&mut input2).expect("Failed to read input!");
    let qty:u32 = input2.trim().parse().expect("Not a valid number");


    if code == "P" {
        let amount = p * qty;
        if amount <= 10_000 {
            println!("Your order costs N{}", amount);
        } else {
            let discount = amount - ((5/100) * amount);
            println!("Your order costs N{}, with a 5% discount", discount);
        }   
    }else if code == "F" {
        let amount = f * qty;
        if amount <= 10_000 {
            println!("Your order costs N{}", amount);
        } else {
            let discount = amount - ((5/100) * amount);
            println!("Your order costs N{}, with a 5% discount", discount);
        }
    }else if code == "A" {
        let amount = a * qty; 
        if amount <= 10_000 {
            println!("Your order costs N{}", amount);
        }else {
            let discount = amount - ((5/100) * amount);
            println!("Your order costs N{}, with a 5% discount", discount);
        }
    }else if code == "E" {
        let amount = e * qty;
        if amount <= 10_000 {
            println!("Your order costs N{}", amount);
        }else {
            let discount = amount - ((5/100) * amount);
            println!("Your order costs N{}, with a 5% discount", discount);
        }
    }else if code == "W" {
        let amount = w * qty;
        if amount <= 10_000 {
            println!("Your order costs N{}", amount);
        }else {
            let discount = amount - ((5/100) * amount);
            println!("Your order costs N{}, with a 5% discount", discount);
        }
    }else if (code != "P") || (code != "F") || (code != "A") || (code != "E") || (code != "W") {
        println!("Invalid code! please input a valid code");
    }
    
    println!("");
    let mut redo = String::new();
    println!("Type quit to exit, or type restart to place a new order: ");
    io::stdin().read_line(&mut redo).expect("Failed to read input!");
    let redo = redo.trim().to_uppercase();

    if redo == "QUIT" {
        break;
    }else if redo == "RESTART" {
        continue;
    }else {
        println!("Invalid input please input either quit or restart !");
    }
}

}    