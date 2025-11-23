use std::io;

fn main() {
    // vector contains the staffs roles, min_years, max_years and level
    let aps_table = vec![
        ("Intern-Paralegal", 1, 2, "APS 1-2"),
        ("Placement", 1, 2, "APS 1-2"),

        ("Administrator", 3, 5, "APS 3-5"),
        ("Research Assistant", 3, 5, "APS 3-5"),
        ("Junior Associate", 3, 5, "APS 3-5"),
        ("Classroom Teacher", 3, 5, "APS 3-5"),

        ("Senior Administrator", 5, 8, "APS 5-8"),
        ("PhD Candidate", 5, 8, "APS 5-8"),
        ("Associate", 5, 8, "APS 5-8"),
        ("Senior Teacher", 5, 8, "APS 5-8"),

        ("Office Manager", 8, 10, "EL1 8-10"),
        ("Post-Doc Researcher", 8, 10, "EL1 8-10"),
        ("Senior Associate 1-2", 8, 10, "EL1 8-10"),
        ("Leading Teacher", 8, 10, "EL1 8-10"),

        ("Director", 10, 13, "EL2 10-13"),
        ("Senior Lecturer", 10, 13, "EL2 10-13"),
        ("Senior Associate 3-4", 10, 13, "EL2 10-13"),
        ("Deputy Principal", 10, 13, "EL2 10-13"),

        // 13 years and above = SES
        ("CEO", 13, 50, "SES"),
        ("Dean", 13, 50, "SES"),
        ("Partner", 13, 50, "SES"),
        ("Principal", 13, 50, "SES"),
    ];

    let mut role = String::new();
    println!("Enter your role:");
    io::stdin().read_line(&mut role).expect("Invalid Input");
    let role = role.trim();

    let mut exp_input = String::new();
    println!("Enter your years of work experience:");
    io::stdin().read_line(&mut exp_input).expect("Invalid Input");
    let years: f32 = exp_input.trim().parse().expect("Enter a number");

    let mut found = false;

    for (r, min, max, aps) in aps_table {
        if r.eq_ignore_ascii_case(role) && years >= min as f32 && years <= max as f32 {
            println!("Staff Role: {}", r);
            println!("Experience: {} years", years);
            println!("APS Level: {}", aps);
            found = true;
            break;
        }
    }

    if !found {
        println!(
            "No APS level found for '{}' with {} years of experience.",
            role, years
        );
    }
}
