fn main() {
	let p:f64 = 1000.0;
	let r:f64 = 1.0;
	let t:f64 = 2.0;

	let a = p * (1.0 + ((r * t) / 100.0 ));
	println!("Amount is NGN{}", a);
	let si = a - p;
	println!(" hence, the interest is NGN{}", si);
}