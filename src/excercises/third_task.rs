// Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in
// a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of
// all people in a department or all people in the company by department, sorted alphabetically.

use std::io;

pub fn add_users() {
    println!("Module third task");
    let mut input_data = String::new();
    io::stdin()
    .read_line(&mut input_data)
    .expect("Не удалось прочитать данные");
    
}
