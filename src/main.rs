mod excercises;

fn run_first_task() {
    println!("First task start");
    {
        let mut v = vec![10, 1, 2, 3, 5, 6];
        let res = excercises::first_task::find_median(&mut v);
        println!("Mean val: {res}");
    }
    {
        let mut v = vec![1, 3, 3, 2, 4];
        let res = excercises::first_task::find_most_often(&mut v);
        println!("Most met val is {res}");
    }
    println!("First task finish");
}

fn run_second_task() {
    println!("Second task start");
    for input_word in ["first", "apple"] {
        let res = excercises::second_task::pig_latin(&input_word.to_string());
        println!("Pig word {res}");
    }
    println!("Second task finish");

}

fn run_third_task() {
    println!("Third task start");
    excercises::third_task::add_users();
    println!("Third task finish");

}

fn main() {
    run_first_task();
    run_second_task();
    run_third_task();
}
