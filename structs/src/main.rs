#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

struct Line {
    start: Point,
    end: Point,
}

fn main() {
    let p1 = Point::new();
    println!("p1 = {:?}", p1);
    let p2 = Point { x: 2.6, y: 5.7 };
    println!("p2 = {:?}", p2);

    println!("Point p1 is at ({}, {})", p1.x, p1.y);
    println!("Point p2 is at ({}, {})", p2.x, p2.y);

    let line = Line { start: p1, end: p2 };
    println!("Line: starts at point ({}, {}), ends at point ({}, {})", line.start.x, line.start.y, line.end.x, line.end.y);
}
