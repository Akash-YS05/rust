struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

struct Square {
    side: u32
}


//impl - we are implementing a method on the struct, in this case its the area method.
impl Square {
    fn area(&self) -> u32 {
        self.side * self.side
    }

    //static method - no need to create an instance of the struct to call this method, calls the struct itself
    fn debug() -> i32 {
        return 1;
    }
}

fn main() {
    let user = User {
        active: false,
        username: String::from("akash@wow"),
        email: String::from("test@xyz.com"),
        sign_in_count: 2
    };

    let sq = Square {
        side: 5
    };

    println!("The area of the square is = {}", sq.area());
    println!("The debug value is = {}", Square::debug());
    println!("The user - {} (currently {}) who is signed with email - {} has signed in {} times", user.username, user.active, user.email, user.sign_in_count);
}