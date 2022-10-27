fn main() {
	//assigning value for toshiba
	let t:f64 = 450000.00 * 2.0;
	// assigning value for Mac
	let m:f64 = 1500000.00 * 1.0;
	// assigning value for HP
	let h:f64 = 750000.00 * 3.0;
	// assigning value for Dell
	let d:f64 = 2850000.00 *3.0;
	// assigning value for acer
	let a:f64 = 250000.00 * 1.0;
	// finding te sum 
	let _sum:f64 = t + m + h + d + a;
	println!("The sum of the sales {}", _sum);

	//finding the average
	let _average:f64 = _sum / 5.0;
	println!("The average is {}", _average);


	}
