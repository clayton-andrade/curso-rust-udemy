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

fn main() {
    let a = 123;
    let b = "hello".to_owned();
    println!("{}", a.format());
    println!("{}", b.format());
    print_it(a);
    print_it(b);
}

fn print_it<T: Printable>(printable: T) {
    println!("{}", printable.format());
}
