//lifetimes in rust mean that the compiler checks how long a reference is valid for
// this is done to prevent dangling pointers

fn main() {
    let str1 = String::from("akassh");
    {
        let str2 = String::from("pa ndey");
        let str3 = longest_string(&str1, &str2);
        println!("Longest string is {}", str3);
    }
    //result will be a dangling pointer if scope of one variable is greater than the other 
}

fn longest_string<'a>(a: &'a String, b: &'a String) -> &'a String {
    if a.len() > b.len() {
        return a;
    }
    return b;
}