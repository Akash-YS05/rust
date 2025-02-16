#[derive(Debug)]

//ENUM - lets us enumerate through a list of possible values


// enum Direction {
//     North,
//     East, 
//     South,
//     West
// }

// fn main() {
//     let dir = Direction::North;
//     println!("Im currently moving to {:?} but later on i might travel to the {:?} direction too", dir, Direction::East);
// }

enum Shape {
    Square(f64),
    Rectangle(f64, f64),
    Circle(f64)
}

fn main() {
    let sq = Shape::Square(9.0);
    let rect = Shape::Rectangle(5.0, 12.0);
    let circle = Shape::Circle(6.0);

    println!("Area of the shape is = {}", get_area(sq));
    println!("Area of the shape is = {}", get_area(rect));
    println!("Area of the shape is = {}", get_area(circle));

    fn get_area(shape: Shape) -> f64 {

        //match - similar to switch case but better as it doesnt need break, can have ranges and can have multiple conditions
        let area= match shape {
            Shape::Square(a) => a*a,
            Shape::Rectangle(a,b)  => a*b,
            Shape::Circle(r) => 3.14 * r * r
        };
        return area;
    }
}