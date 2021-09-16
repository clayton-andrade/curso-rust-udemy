use std::sync::{Arc, Mutex};
use std::thread;

struct Person {
    name: Arc<String>,
    state: Arc<Mutex<String>>,
}

impl Person {
    fn new(name: Arc<String>, state: Arc<Mutex<String>>) -> Self {
        Person { name, state }
    }

    fn greet(&self) {
        let mut state = self.state.lock().unwrap();
        state.clear();
        state.push_str("excited");
        
        println!("Hi, my name is {} and I am {}.", self.name, state);
    }
}

fn main() {
    let name = Arc::new("John".to_owned());
    let state = Arc::new(Mutex::new("bored".to_owned()));
    let p = Person::new(name.clone(), state.clone());
    let t = thread::spawn(move || {
        p.greet();
    });
    println!("Name: {}, state: {}", name, state.lock().unwrap());
    t.join().unwrap();
}

