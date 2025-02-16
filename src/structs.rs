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
    println!("The user - {} (currently {}) who is signed with email - {} has signed in {} times", user.username, user.active, user.email, user.sign_in_count);
}