#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct Point2<T, V> {
    x: T,
    y: V,
}

fn main() {
    let p1 = Point { x: 3, y: 5 };
    let p2: Point<f64> = Point { x: 2.6, y: 5.9 };
    let p3 = Point::<u8> { x: 123, y: 255 };
    let p4 = Point2 { x: 10, y: 4.5 };
    let p5: Point2<f32, u32> = Point2 { x: 22.6, y: 88 };
    println!("p1 = ({}, {}), p2 = ({}, {})", p1.x, p1.y, p2.x, p2.y);
    println!("{:?}", p3);
    println!("{:?}, {:?}", p4, p5);
}
