// Chidi's Menu and Expense Calculator
use std::io;
fn main() {

    let mut input1 = String::new();
    let mut input2 = String::new();

    println!(" Code       Menu                                 Price (N)");
    println!("  P          Fried Rice and chicken               3,000");
    println!("  F          Poundo Yam/Edinkaiko Soup            3,200");
    println!("  A          Amala and Ewedu Soup                 2,500");
    println!("  E          Eba and Egusi Soup                   2,000");
    println!("  W          White Rice and Stew                  2,500");

    println!("Enter the Code of Your Preferred Item ");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let code = input1.trim();

    println!("Enter the quantity you will be buying");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let quantity: f32 = input2.trim().parse().expect("Not a valid number");
    
    let price: f32 = match code {
        "P" => 3000.0,
        "F" => 3200.0,
        "A" => 2500.0,
        "E" => 2000.0,
        "W" => 2500.0,
        _ => {
            println!("Invalid code entered!"); return;
        }
    };

    let mut cost = price * quantity;

    if cost > 10000.0 {
        let discount = 0.05 * cost;
        cost = cost - discount;
        println!("Your discount is N{}", discount);
    }

    println!("Your cost is N{}", cost);
    println!("Hope you enjoy the rest of your day");
}
