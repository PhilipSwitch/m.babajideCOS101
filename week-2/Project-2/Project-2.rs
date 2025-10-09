fn main(){
	/*let  t represent Toshiba
	  let  m represent Mac
	  let  h represent HP
	  let  d represent Dell
	  let  a represent Acer
	*/
	let t:f64 = 2.0 *450_000.00;
	let m:f64 = 1.0 *1_500_000.00;
	let h:f64 = 3.0 *750_000.00;
	let d:f64 = 3.0 *2_850_000.00;
    let a:f64 = 1.0 *250_000.00;


    // Average (A)
    
	let average = t+m+h+d+a/5.0;
    println!("Average is {}", average);

    // Sum (S)
    let sum = t+m+h+d+a;
    println!("Sum is {}", sum);
    
}