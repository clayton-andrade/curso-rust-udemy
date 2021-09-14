trait Printable {
    fn format(&self) -> String;
}

impl Printable for i32 {
    fn format(&self) -> String {
        format!("i32: {}", self)
    }
}

impl Printable for String {
    fn format(&self) -> String {
        format!("String: {}", self)
    }
}

trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

struct Square {
    side: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

fn main() {
    let a = 123;
    let b = "hello".to_owned();
    println!("{}", a.format());
    println!("{}", b.format());
    print_it(&a);
    print_it(&b);

    println!();

    let shapes: [&dyn Shape; 4] = [
        &Circle { radius: 1.0 },
        &Square { side: 3.5 },
        &Square { side: 2.2 },
        &Circle { radius: 2.5 }
    ];

    for (i, shape) in shapes.iter().enumerate() {
        println!("Area of shape #{}: {}", i + 1, shape.area());
    }
}

fn print_it(printable: &dyn Printable) {
    println!("{}", printable.format());
}
