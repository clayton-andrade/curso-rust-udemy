trait Summable<T> {
    fn sum(&self) -> T;
}

impl Summable<i32> for Vec<i32> {
    fn sum(&self) -> i32 {
        self.iter().fold(0, |acc, x| acc + x)
    }
}

trait Animal {
    fn new(name: String) -> Self;
    
    fn name(&self) -> String;

    fn talk(&self) {
        println!("{} cannot talk", self.name());
    }
}

struct Human {
    name: String,
}

impl Animal for Human {
    fn new(name: String) -> Self {
        Human { name }
    }
    
    fn name(&self) -> String {
        self.name.to_owned()
    }

    fn talk(&self) {
        println!("Hi, my name is {}", self.name());
    }
}

struct Cat {
    name: String,
}

impl Animal for Cat {
    fn new(name: String) -> Self {
        Cat { name }
    }

    fn name(&self) -> String {
        self.name.to_owned()
    }
}

fn main() {
    let human = Human::new("Clayton".to_owned());
    println!("{}", human.name());
    human.talk();
    let cat: Cat = Animal::new("Fluffy".to_owned());
    println!("{}", cat.name());
    cat.talk();

    let v = vec![1, 2, 3, 4, 5];
    println!("Sum = {}", v.sum())
}
