fn main() {
	let t: f64 = 450_000.0;
	let m: f64 = 1_500_000.0;
	let hp: f64 = 750_000.0;
	let d : f64 = 2_850_000.0;
	let a: f64 = 250_000.0;

	let sum = ( 2.0 * t ) + m + ( 3.0 * hp ) + ( 3.0 * d ) + a;
	println!("Sum is {}", sum);

	let average = sum / 10.0;
	println!("Average is {}", average);
}