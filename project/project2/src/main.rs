use std::io;

// A compound data type (STRUCT)
struct Candidate {
    name: String,
    years: u32,
}

fn main() {
    let mut candidates: Vec<Candidate> = Vec::new();

    println!("-- EY Nigeria Interview Experience Checker --");
    println!("Enter number of candidates:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Invalid input");
    let count: usize = input.trim().parse().expect("Enter a number");

    // Collect candidate records
    for i in 1..=count {
        println!("\nEnter name of candidate {}:", i);
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Invalid name");

        println!("Enter years of programming experience for {}:", name.trim());
        let mut years_input = String::new();
        io::stdin()
            .read_line(&mut years_input)
            .expect("Invalid years");
        let years: u32 = years_input.trim().parse().expect("Enter a number");

        candidates.push(Candidate {
            name: name.trim().to_string(),
            years,
        });
    }

    // Find the candidate with the highest experience
    let mut most_experienced = &candidates[0];

    for candidate in &candidates {
        if candidate.years > most_experienced.years {
            most_experienced = candidate;
        }
    }

    // Output result
    println!("\n-------------------------------------------");
    println!("The person with the highest experience is:");
    println!("Name: {}", most_experienced.name);
    println!("Years of Programming Experience: {}", most_experienced.years);
    println!("-------------------------------------------");
}