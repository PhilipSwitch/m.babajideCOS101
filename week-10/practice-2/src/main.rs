fn main() {
    
    let v = vec![10,20,30];

    let v2 = v;

    display(v2.clone());

    println!("In the main {:?}",v2);

}

fn display(v:Vec<i32>) {

    println!("inside display {:?}",v);
}