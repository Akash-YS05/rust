use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    //args[0] is the name of program
    let c1 = &args[1];
    let c2 = &args[2];

    println!("First argument is {c1}");
    println!("Second argument is {c2}")
}
