#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y:f64) -> Self {
        Point{ x, y }
    }
}

#[derive(Debug)]
struct Circle {
    center: Point,
    radius: f64,
}

impl Circle {
    fn new(center: Point, radius: f64) -> Self {
        Circle{ center, radius }
    }

    fn area(&self) -> f64 {
        self.radius.powf(2.0) * std::f64::consts::PI
    }
}

fn main() {
    let center = Point::new(0.0, 0.0);
    let circ = Circle::new(center, 2.0);

    println!("The center of the circle is: {:?}", circ.center);
    println!("The radius of the circle is: {:?}", circ.radius);
    println!("The area of the circle is: {:?}", circ.area());
}