// Rust program for finding root

use std::io; 

fn main() {
    println!("Quadratic Equation Solver: ax^2 + bx + c = 0");

    // Input a
    println!("Enter value for a:");
    let mut input_a = String::new();
    io::stdin().read_line(&mut input_a).expect("Failed");
    let a: f64 = input_a.trim().parse().expect("Not a number");

    // Input b
    println!("Enter value for b:");
    let mut input_b = String::new();
    io::stdin().read_line(&mut input_b).expect("Failed");
    let b: f64 = input_b.trim().parse().expect("Not a number");

    // Input c
    println!("Enter value for c:");
    let mut input_c = String::new();
    io::stdin().read_line(&mut input_c).expect("Failed");
    let c: f64 = input_c.trim().parse().expect("Not a number");

    // Calculate discriminant
    let d = b * b - 4.0 * a * c;

    if d > 0.0 {
        // Two real roots
        let root1 = (-b + d.sqrt()) / (2.0 * a);
        let root2 = (-b - d.sqrt()) / (2.0 * a);
        println!("Two Real Roots:");
        println!("x1 = {}", root1);
        println!("x2 = {}", root2);

    } else if d == 0.0 {
        // One real root
        let root = -b / (2.0 * a);
        println!("One Real Root:");
        println!("x = {}", root);

    } else {
        // Complex roots
        let real_part = -b / (2.0 * a);
        let imag_part = (-d).sqrt() / (2.0 * a);
        println!("Complex Roots:");
        println!("x1 = {} + {}i", real_part, imag_part);
        println!("x2 = {} - {}i", real_part, imag_part);
    }
}