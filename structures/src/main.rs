// An attribute to hide warnings for unused code.
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[derive(Debug)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    fn area(&self) -> f32 {
        let side: f32 = self.top_left.y - self.bottom_right.y;
        let top: f32 = self.bottom_right.x - self.top_left.x;
        top*side
    }
}

fn rect_area(rectangle: &Rectangle) -> f32 {
    let side: f32 = rectangle.top_left.y - rectangle.bottom_right.y;
    let top: f32 = rectangle.bottom_right.x - rectangle.top_left.x;
    top*side
}

fn square(point: &Point, scale: &f32) -> Rectangle {
    Rectangle{
        top_left: Point { x: point.x, y: point.y },
        bottom_right: Point { x: (point.x+scale), y: (point.y-scale) },
    }
}

fn main() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, ..point };

    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point { x: left_edge, y: top_edge } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    let test_rect = Rectangle {
        top_left: Point { x: (0_f32), y: (1_f32) },
        bottom_right: Point{ x: (1_f32), y: (0_f32) },
    };
    println!("Area of testRect is: {}", rect_area(&test_rect));
    println!("IMPL of testRect is: {}", test_rect.area());
    
    let test_point = Point {
        x: 5.0,
        y: 5.0,
    };
    let test_scale: f32 = 3.0;
    println!("Square with Point {:?} and scale {} is: {:?}", test_point, test_scale, square(&test_point, &test_scale));
}