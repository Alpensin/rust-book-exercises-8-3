// Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in
// a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of
// all people in a department or all people in the company by department, sorted alphabetically.

use std::{collections::HashMap, io};

fn add_users(deps: &mut HashMap<String, Vec<String>>) {
    println!("Module third task");
    loop {
        println!(
            "Enter data. Expected \"Add <employee> to <department>\". Command <Stop> to finish"
        );
        let mut input_data = String::new();
        io::stdin()
            .read_line(&mut input_data)
            .expect("Enter new employee. Expect \"Add <employee> to <department>\" ");
        if input_data.trim() == "Stop" {
            break;
        }
        let splitted_data: Vec<&str> = input_data.trim().split(" to ").collect();
        if splitted_data.len() != 2 {
            println!("You entered wrong data. Expected \"Add <employee> to <department>\"");
            continue;
        }
        let mut spl_iterator = splitted_data.iter();
        let employee: &Option<&&str> = &spl_iterator.next();
        match employee {
            Some(_) => (),
            None => {
                println!("You entered wrong employee. Expected \"Add <employee> to <department>\"");
                continue;
            }
        };
        let employee = employee.unwrap();
        let employee = employee.trim_start_matches("Add ");
        let department = &spl_iterator.next();
        println!("employee={employee}");

        match department {
            Some(_) => (),
            None => {
                println!(
                    "You entered wrong department. Expected \"Add <employee> to <department>\""
                );
                continue;
            }
        };
        let department = department.unwrap();

        println!("department={department}");
        let cur_arr = deps.entry(String::from(*department)).or_insert(vec![]);
        cur_arr.push(String::from(employee));
    }
}
fn fill_new_users(deps: &mut HashMap<String, Vec<String>>) {
    loop {
        println!(
            "Enter data. Expected: list <department name>(Optional). Command <Stop> to finish"
        );
        let mut input_data = String::new();
        io::stdin()
            .read_line(&mut input_data)
            .expect("Enter command list or list <department");
        if input_data.trim() == "Stop" {
            break;
        }
        let splitted_data: Vec<&str> = input_data.trim().split(" ").collect();

        match splitted_data.len() {
            1 => {
                let command = splitted_data[0];
                println!("command={command}");
                let mut all_employees = vec![];
                for employees in deps.values() {
                    for employee in employees {
                        all_employees.push(employee);
                    }
                }
                all_employees.sort();
                for employee in all_employees {
                    println!("{employee}");
                }
            }
            2 => {
                let command = splitted_data[0];
                let dep = splitted_data[1];
                println!("command={command}, dep={dep}");
                let employees = deps.get(dep);
                match employees {
                    Some(employees) => {
                        let mut cur_employees = vec![];
                        for emp in employees {
                            cur_employees.push(emp);
                        }
                        cur_employees.sort();
                        for emp in cur_employees {
                            println!("{emp}");
                        }
                    }
                    None => println!("No employees found for this department"),
                }
            }
            _ => {
                println!("You entered wrong data. Expected: list <department name>(Optional)");
                continue;
            }
        }
    }
}
pub fn manage_employees(deps: &mut HashMap<String, Vec<String>>) {
    add_users(deps);
    fill_new_users(deps);
}
