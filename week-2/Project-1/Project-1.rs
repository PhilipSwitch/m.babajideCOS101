fn main() {
	/*let p represent principal
	  let r represent Rate
	  let n represent number of years
	*/
	let p = 520_000_000.00;
	let r:f64 = 10.0;
	let n:f64 = 5.0;
	// let a reprsent Amount (A)
	let a = p*(1.0+(r/100.0)).powf(n);
	println!("Amount is {}", a);

	// let ci represent compouund interest
	let ci = a-p;
    println!("Compound Interest is {}", ci);
    
}