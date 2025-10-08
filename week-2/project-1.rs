fn main() {
	let p:f64 = 520000000.00;
	let r:f64 = 10.00;
	let t:f64 = 5.00;

	let amount = p * ((1.0 + (r / 100.0)).powf(t));
	let cinterset = amount - p;
	println!("The amount is NGN{:.2}, while the compound interest is NGN{:.2}", amount, cinterset); 
}