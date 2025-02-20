mod conditions_loops;
mod own_bor;
mod structs;
mod enums;
fn main() {
    fn get_len(s: &str) -> usize {
        s.chars().count()
    }

    let arr: [i8; 5] = [2,3,4,5,6];    //cant grow, size needs to be known at compile time
    let mut vect: Vec<i8> = vec![6,7,8,9,10]; //can grow, size can be changed at runtime
    vect.push(11);
    let word = String::from("Hello, World! ");
    let isx: bool = false;
    let x: i32 = 5;
    let mut y: i32 = 10;
    y+=10;

    //the ! indicates that this is a macro. macros are used to generate code
    println!("{}", word);
    println!("Length of string = {}", get_len(&word));
    println!("Value of x (unmutable) is: {}", x);
    println!("Value of y (mutable) is: {}", y+10);
    println!("isx current status is {}", isx);
    println!("Array is : {:?}", arr);
    println!("Vector is : {:?}", vect);
    println!("Length of array is: {}", arr.len());
   
}