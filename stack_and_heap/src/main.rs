use std::mem;

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

fn main() {
    // stack
    let x = 5;
    println!("x = {}", x);

    // heap
    let y = Box::new(5);
    println!("y = {}", y);

    let p1 = origin();
    println!("p1 = {:?}", p1);

    let p2 = Box::new(Point {x: 4.5, y: 8.1});
    println!("p1 = {:?}", p2);

    println!("p1 takes up {} bytes", mem::size_of_val(&p1));
    println!("p2 takes up {} bytes", mem::size_of_val(&p2));

    let p3 = *p2;
    println!("x = {}", p3.x);
    println!("y = {}", p3.y);
}

fn origin() -> Point {
    Point {x: 0.0, y: 0.0}
}