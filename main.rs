// Program or employee incentive

use std::io;
fn main() {
println!("EMPLOYEE DATABASE");
let mut input1 =String::new();
let mut input2 =String::new();

println!("\nEnter your experience level (experienced or unexperienced:)");    
io::stdin().read_line (&mut input1).expect("Faild to read input");
let experience=input1.trim().to_lower();


println!("\nEnter your age");    
io::stdin().read_line (&mut input2).expect("Faild to read input");
let age:u232=input2.trim().parse().expect("Invalid data type")

let incentive :i32;

if experience == "experience" {
    if age >= 40 {
        print!("\nThe annual incentive for this employee is #1,560,000");
    } else if age >= 30 && age < 40 {
        println!("\nThe annual incentive for this employee is #1,480,000");  
    } else if age < 28 {
        println!("\nThe annual incentive for this employeeis #1,300,000 ");} else {
            println!("\nThe annual incentive for this employee is #1,000,000")
        }
} else if experience == 
"unexperienced" || eperience ==
"inexperience" {
    println!("\nThe annual incentive for this employee is #100,000");
} else {
    println!("\nInvaldid experience input. Please enter 'experienced" or 'unexperienced'");
}
}


