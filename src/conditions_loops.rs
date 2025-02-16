fn main() {
    let x: i8 = 32;
    let num: bool = is_even(x);
    if num {
        println!("Number is even {}", x);
    } else {
        println!("Number is odd");
    }


    pub fn is_even(x: i8) -> bool {
        return x % 2 == 0;
    }
}