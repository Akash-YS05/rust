use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok(); //read from env and give it to dotenv
    let var = env::var("SOME_SECRET"); //returns Result<String, varError>

    match var {
        Ok(str) => println!("{str:?}"),
        Err(error) => println!("{error:?}")
    }
}