//Error handling is done by Result which is an enum with two variants - Ok and Err
//Option enum is used to handle null values as Rust does not have null variable
use std::fs::File;

fn main() {

    let name = find_first(String::from("monaco"));

    match name {
        Some(value) => println!("Index is {value}"),
        None => println!("No value found")
    }
    // let test_file = File::open("hello.txt");

    // let file = match test_file {
    //     Ok(file) => file,
    //     Err(error) => panic!("Some error occured! {error:?}")
    // };


}

fn find_first(name: String) -> Option<i32> {     //<i32> is a generic which is used to basically tell the compiler that the output of the function will be an integer. usually helps avoiding code repitition
    for (index, char) in name.chars().enumerate() {
        if char == 'a' {
            return Some(index as i32);    //if the output is desirable then use Some variant of Option enum
        } 
    }
    return None;    //if the output is not desirable then use None variant of Option enum

}