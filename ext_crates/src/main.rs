// use chrono::prelude::*; - import all the modules of chrono crate
// use chrono::{Utc, Local};

use dotenv::dotenv;
use std::env;

fn main() {
    // let utc = Utc::now(); //now() is a static function
    // let local = Local::now();
    // println!("UTC is: {utc:?}");
    // println!("Local is: {local:?}");

    dotenv().ok(); //read from env and give it to dotenv
    let var = env::var("SOME_SECRET"); //returns Result<String, varError>

    match var {
        Ok(str) => println!("{str:?}"),
        Err(error) => println!("{error:?}")
    }
}
