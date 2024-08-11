// Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company; for example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
use std::collections::HashMap;
use std::io;

fn main() {
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();

    departments.insert(
        "sales".to_string(),
        vec!["John".to_string(), "Kevin".to_string()],
    );
    departments.insert(
        "engineering".to_string(),
        vec!["Salomé".to_string(), "Lebron".to_string()],
    );

    loop {
        println!("Please enter a command (e.g., 'Add Sally to Engineering', or 'List Department Sales', or 'List All'):");

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let trimmed_input = input.trim();
                if trimmed_input.is_empty() {
                    continue;
                }

                let parts: Vec<&str> = trimmed_input.split_whitespace().collect();

                let command: String = parts[0].to_lowercase();
                match command.as_str() {
                    "add" => {
                        if parts.len() != 4 {
                            println!(
                                "Add command needs to be exactly 4 words (e.g: Add John Sales)"
                            );
                            continue;
                        }
                        let name: String = parts[1..parts.len() - 2].join(" ");
                        let department: String = parts[parts.len() - 1].to_lowercase();
                        add_employee(&mut departments, &name, department);
                    }
                    "list" => {
                        // "list departement 'name'" command
                        if parts.len() == 3 && parts[1].eq_ignore_ascii_case("department") {
                            list_department(&departments, parts[2]);
                        } else if parts.len() == 2 && parts[1].eq_ignore_ascii_case("all") {
                            // "list all" command
                            list_all(&departments);
                        } else {
                            println!("Invalid list command. Try again.");
                        }
                    }
                    _ => {
                        println!("Unknown command. Please use 'Add' or 'List'.");
                    }
                }
            }
            Err(_) => println!("Error reading input. Please try again."),
        }
    }
}

fn add_employee(departments: &mut HashMap<String, Vec<String>>, name: &str, department: String) {
    departments
        .entry(department.to_string())
        .or_insert_with(Vec::new)
        .push(name.to_string());
    println!("Added {} to {}", name, department);
    list_all(departments);
}

fn list_department(departments: &HashMap<String, Vec<String>>, department: &str) {
    match departments.get(department) {
        Some(employees) => {
            let mut sorted_employees: Vec<String> = employees.clone();
            sorted_employees.sort();
            println!("Employees in {}: {:?}", department, sorted_employees);
        }
        None => println!("Department {} does not exist.", department),
    }
}

fn list_all(departments: &HashMap<String, Vec<String>>) {
    for (department, employees) in departments {
        let mut sorted_employees: Vec<String> = employees.clone();
        sorted_employees.sort();
        println!("{}: {:?}", department, sorted_employees);
    }
}
