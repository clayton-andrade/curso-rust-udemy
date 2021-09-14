struct Person {
    name: String,
}

impl Person {
    // fn new(name: &str) -> Self {
    //     Person { name: name.to_owned() }
    // }
    fn new<T>(name: T) -> Self
    where
        T: Into<String>
    {
        Person { name: name.into() }
    }
}

fn main() {
    let p1 = Person::new("Clayton");
    println!("{}", p1.name);
    let name = "Heloiza".to_owned();
    let p2 = Person::new(name);
    println!("{}", p2.name);
}
