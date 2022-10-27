fn main() {
	
	//assigning the principal value
	let p:f64 = 520000000.0;
	// assigning the rate value
	let r:f64 = 10.0;
	// assigning the time value
	let t:f64 = 5.0;

	// putting in the formula for Amount
	let amount:f64  = p * ((1.0 + (r/100.0)).powf(t));
	println!("Amount is {}",amount);
	// Assigning the compound interest
	let compound_interest:f64 = amount - p;
	println!("The compound interest is {}", compound_interest);


}