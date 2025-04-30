use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]

struct User {
    username: String,
    password: String
}

fn main() {
    let u =  User {
        username: String::from("akash"),
        password: String::from("shdiwww")
    };

    let serialized_string = serde_json::to_string(&u);      //converts the struct to a JSON string
    match serialized_string {
        Ok(str) => println!("Serialized: {}", str),
        Err(_) => println!("Could not serialize the object")
    }
    let s = String::from("{\"username\":\"akash\",\"password\":\"shdiwww\"}");
    let deserialized_string: Result<User, serde_json::Error> = serde_json::from_str(&s);
    println!("Deserialized Person: {:?}", deserialized_string.unwrap());

    // match deserialized_string {
    //     Ok(user) => println!("Deserialized: {:?}", user),
    //     Err(_) => println!("Could not deserialize the object")
    // }
}
