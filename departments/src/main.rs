use std::collections::HashMap;
use std::io;

enum Command {
    Add(String, String),
    PrintDept(String),
    PrintOrganization,
    Quit,
    Invalid,
}

struct Organization {
    department_employees: HashMap<String, Vec<String>>,
}

impl Organization {
    fn new() -> Organization {
        Organization {
            department_employees: HashMap::new(),
        }
    }

    fn add_person_to_department(&mut self, person: String, dept: String) {
        let vector = self
            .department_employees
            .entry(dept)
            .or_insert_with(Vec::new);
        vector.push(person);
        vector.sort_unstable();
    }

    fn print_dept(&self, dept: String) {
        let vector = match self.department_employees.get(&dept.to_lowercase()) {
            Some(vec) => vec,
            None => {
                println!("There is no department called {}.", &dept);
                return;
            }
        };
        println!("{}: {}", &dept, vector.join(", "));
    }

    fn print_company(&self) {
        let mut keys: Vec<_> = self.department_employees.keys().collect();
        keys.sort_unstable();
        for dept in keys {
            let vector = match self.department_employees.get(&dept.to_lowercase()) {
                Some(vec) => vec,
                None => continue,
            };
            println!("{}: {}", &dept, vector.join(", "));
        }
    }
}

fn parse_input(command: &str) -> Command {
    let words: Vec<&str> = command.split_whitespace().collect();
    match words.len() {
        4 => {
            if words[0].to_lowercase() == "add" && words[2].to_lowercase() == "to" {
                Command::Add(words[1].to_string(), words[3].to_string().to_lowercase())
            } else {
                Command::Invalid
            }
        }
        2 => {
            if words[0].to_lowercase() == "print" {
                if words[1].to_lowercase() == "company" {
                    Command::PrintOrganization
                } else {
                    Command::PrintDept(words[1].to_lowercase().to_string())
                }
            } else {
                Command::Invalid
            }
        }
        1 => {
            if words[0].to_lowercase() == "quit" {
                Command::Quit
            } else {
                Command::Invalid
            }
        }
        _ => Command::Invalid,
    }
}

fn main() {
    let mut org = Organization::new();
    println!("Please enter a command. Options:\n1. add x to y\n2. print department\n3. print company\n4. quit");
    loop {
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line.");
        match parse_input(&command) {
            Command::Add(person, dept) => org.add_person_to_department(person, dept),
            Command::PrintDept(dept) => org.print_dept(dept),
            Command::PrintOrganization => org.print_company(),
            Command::Quit => break,
            Command::Invalid => println!("Invalid command, please try again."),
        };
    }
}
