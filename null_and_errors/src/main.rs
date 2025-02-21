//Error handling is done by Result which is an enum with two variants - Ok and Err

use std::fs::File;

fn main() {
    let test_file = File::open("hello.txt");

    let file = match test_file {
        Ok(file) => file,
        Err(error) => panic!("Some error occured! {error:?}")
    };
}
