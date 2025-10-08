fn main() {
	let tosh:f64 = 450000.0;
	let toshn:f64 = 2.0;
	let mac:f64 = 1500000.0;
	let macn:f64 = 1.0;
	let hp:f64 = 750000.0;
	let hpn:f64 = 3.0;
	let dell:f64 = 2850000.0;
	let delln:f64 = 3.0;
	let acer:f64 = 250000.0;
	let acern:f64 = 1.0;

	let sum = tosh + mac + hp + dell + acer;
	let avsum = (tosh * toshn) + (mac * macn) + (hp * hpn) + (dell * delln) + (acer * acern);
	let total = toshn + macn + hpn + delln + acern;
	let average = avsum / total;

	println!("The sum is NGN{:.2} while the average is NGN{:.2}.", sum, average);
}