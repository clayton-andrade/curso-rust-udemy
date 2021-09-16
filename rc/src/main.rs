use std::rc::Rc;

struct Person {
    name: Rc<String>,
}

impl Person {
    fn new(name: Rc<String>) -> Self {
        Person { name }
    }

    fn greet(&self) {
        println!("Hi, my name is {}.", self.name);
    }
}

fn main() {
    let name = Rc::new("John".to_owned());
    println!("Name: {}, name has {} strong pointers", name, Rc::strong_count(&name));
    {
        let p = Person::new(name.clone());
        p.greet();
        println!("Name: {}, name has {} strong pointers", name, Rc::strong_count(&name));
    }
    println!("Name: {}", name);
    println!("Name: {}, name has {} strong pointers", name, Rc::strong_count(&name));
}
