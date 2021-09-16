use std::sync::Arc;
use std::thread;

struct Person {
    name: Arc<String>,
}

impl Person {
    fn new(name: Arc<String>) -> Self {
        Person { name }
    }

    fn greet(&self) {
        println!("Hi, my name is {}.", self.name);
    }
}

fn main() {
    let name = Arc::new("John".to_owned());
    let p = Person::new(name.clone());
    let t = thread::spawn(move || {
        p.greet();
    });
    println!("Name: {}", name);
    t.join().unwrap();
}
