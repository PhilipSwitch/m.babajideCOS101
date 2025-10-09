fn main() {
	/*let p represent principal(P)
	  let r represent rate (R)
	  let n represent number of years
	*/
	let p:f64 = 510_000.0;
	let r:f64 = 5.0;
	let n:f64 = 3.0;

	// let a represent Amount (A)
	let a = p*(1.0+(r/100.0)).powf(n);
	println!("Amount is {}", a);

    //let d represent depreciation
    let d = p-a;
    println!("Depreciation is {}", d);

}