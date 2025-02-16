fn main() {
    let str1 = String::from("Hi there, how are you?");
    let mut str2 = String::from("Hello!");

    let len = calculate_length(&str1);
    println!("Length of string is: {}", len);

    append_text(&mut str2);


    //pass by reference allows borrowing the value while the ownership remains with the original variable
    /*  Some rules:
        1. At any given time, you can have either one mutable reference or any number of immutable references
        2. References must always be valid
        
     */
    fn calculate_length(s: &String) -> usize {
        return s.chars().count();
    }
    
    fn append_text(s: &mut String) {
        s.push_str(", World!");
        println!("{}", s);
        
    }
}

