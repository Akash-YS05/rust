trait Shape {
    fn area(&self) -> u32;
}

struct Rect {
    height: u32,
    width: u32
}

impl Shape for Rect {
    fn area(&self) -> u32 {
        return self.height * self.width;
    }
}

fn main() {
    //println! is a declarative macro, changes the written code with a different code during compile time
    println!("Hello, world!");
    let r = Rect {
        height: 20,
        width: 7
    };

    println!["Area: {}", get_area(r)];
}

fn get_area(s: impl Shape) -> u32 {
    return s.area();
 }