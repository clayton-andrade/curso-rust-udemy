struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn from(x: f64, y: f64) -> Self {
        Point { x, y }
    }
}

struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn from(start: Point, end: Point) -> Self {
        Line { start, end }
    }

    fn len(&self) -> f64 {
        let dx = self.start.x - self.end.x;
        let dy = self.start.y - self.end.y;
        (dx * dx + dy * dy).sqrt()
    }
}

fn main() {
    let p1 = Point::from(3.67, 4.58);
    let p2 = Point::from(5.55, 7.91);
    let my_line = Line::from(p1, p2);
    println!("length: {}", my_line.len());
}
