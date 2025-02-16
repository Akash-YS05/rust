fn main() {
    let x: i8 = 32;
    let num: bool = is_even(x);
    if num {
        println!("Number is even {}", x);
    } else {
        println!("Number is odd");
    }

    pattern(5);


    //equivalent code for (int i = 0; i<10; i+=2)
    for z in (0..10).step_by(2) {
        println!("{}", z);
    }

    println!("{}", fib(5));

    fn fib(x: i32) -> i32 {
        let mut first = 0;
        let mut second = 1;
        
        for _i in 1..x - 1 {
            let temp = second;
            second = first + second;
            first = temp;
        }

        return second;
    }


    pub fn is_even(x: i8) -> bool {
        return x % 2 == 0;
    }


    //no arrow after data type means it returns void
    pub fn pattern(x: i8) {
        for i in 1..x+1 {
            for _j in 0..i {
                print!("*");
            }
            println!();
        }
    }
}