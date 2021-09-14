struct Creature {
    name: String,
}

impl Creature {
    fn new(name: &str) -> Self {
        println!("Creature {} created.", name);
        Creature { name: name.into() }
    }
}

impl Drop for Creature {
    fn drop(&mut self) {
        println!("Creature {} dropped.", self.name);
    }
}

fn main() {
    let elfo = Creature::new("Elfo");
    {
        let hobbit = Creature::new("Hobbit");
        println!("{}", hobbit.name);
    }
    println!("{}", elfo.name);
}
