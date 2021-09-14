trait Animal {
    fn new(name: String) -> Self where Self: Sized;
    
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

enum Creature {
    Human(Human),
    Cat(Cat),
}

fn main() {
    let h1 = Human::new("Clayton".to_owned());
    let h2 = Human::new("Heloiza".to_owned());
    let c1 = Cat::new("Missy".to_owned());
    let c2 = Cat::new("Fluffy".to_owned());
    
    let animals: Vec<&dyn Animal> = vec![&h1, &c1, &h2, &c2];

    for animal in animals {
        println!("{}:", animal.name());
        animal.talk();
    }

    println!();

    let mut creatures = Vec::new();
    creatures.push(Creature::Human(Human { name: "Clayton".to_owned() }));
    creatures.push(Creature::Cat(Cat { name: "Fluffy".to_owned() }));
    for creature in &creatures {
        match creature {
            Creature::Human(h) => h.talk(),
            Creature::Cat(c) => c.talk(),
        }
    }

    println!();

    let mut box_animals = Vec::<Box<dyn Animal>>::new();
    box_animals.push(Box::new(Human::new("Heloiza".to_owned())));
    box_animals.push(Box::new(Cat::new("Missy".to_owned())));
    for animal in &box_animals {
        animal.talk();
    }
}
