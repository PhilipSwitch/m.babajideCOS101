fn main() {
    
    let emp1 = Employee {
        company:String::from("Microsoft Corporation"),
        ceo:String::from("Satya Nadella"),
        age:56
    };
    let emp2 = Employee {
        company:String::from("Google Inc."),
        ceo:String::from("Sundai Pichai"),
        age:51
    };
    //Pass emp1 and wmp2 to display()
    display(emp1);
    display(emp2);
}

fn display(emp:Employee){
    println!("Name is : {} company is {} age is {}", emp.ceo,emp.company,emp.age);

}

struct Employee {
    ceo:String,
    company:String,
    age:u32

}