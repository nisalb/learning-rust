use std::io::{self, Write};

use employees::{operations::Operation, Company};

fn main() {
    println!("Welcome to nanoDB");
    println!("Following commands are supported:");
    println!("\t* add <name> to <department>");
    println!("\t* remove <name> from <department>");
    println!("\t* transfer <name> from <department> to <department>");
    println!("\t* list <department>");

    let mut company = Company::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Unable to read the input");

        let operation = company.operation(&input).expect("invalid operation");

        match operation {
            Operation::Add(person, department) => company.add(person, department),
            Operation::Remove(person, department) => company.remove(&person, department),
            Operation::Transfer(person, from, to) => company.transfer(person, from, to),
            Operation::ListAll(department) => {
                let employees = company.list_all(&department);
                println!("{}: [{}]", department, employees.join(", "));
            }
            Operation::End => break,
        }
    }
}
