fn main() {
	//assigning princinpal value
	let p:f64 = 210000.0;
	// assigning rate value 
	let r:f64 = 5.0;
	// assigning future value
	let t:f64 = 3.0;

	// Amount

	let _amount:f64 = p * ((1.0 - r/100.0).powf(t));
	println!("The Amount is {}", _amount);

	//Depreciation
	let _depreciation:f64 = p - _amount;
	println!("The depreciation {}", _depreciation);

	}