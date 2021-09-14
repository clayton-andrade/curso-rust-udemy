use std::fmt::Debug;

trait Shape {
    fn area(&self) -> f64;
}

#[derive(Debug)]
struct Square {
    side: f64,
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

#[derive(Debug)]
struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius) 
    }
}

fn main() {
    let circle = Circle { radius: 3.67 };
    let square = Square { side: 8.42 };
    print_info(&circle);
    print_info(&square);
}

// fn print_info(shape: impl Shape + Debug) {
// fn print_info<T: Shape + Debug>(shape: &T) {
fn print_info<T>(shape: &T)
where
    T: Shape + Debug
{
    println!("{:?}", shape);
    println!("Area: {:.2}", shape.area());
}