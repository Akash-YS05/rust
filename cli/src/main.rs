use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    // //args[0] is the name of program
    // let c1 = &args[1];
    // let c2 = &args[2];

    // println!("First argument is {c1}");
    // println!("Second argument is {c2}");

    let c1: i32 = args[1].parse().expect("Please provide a valid integer for the first argument");
    let c2: i32 = args[2].parse().expect("Please provide a valid integer for the second argument");

    let sum: i32 = c1 + c2;

    println!("First argument is {c1}");
    println!("Second argument is {c2}");
    println!("Sum = {sum}");

}
