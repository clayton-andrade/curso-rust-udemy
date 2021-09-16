#[derive(Debug)]
struct Person {
    name: String,
}

impl Person {
    fn ref_name(&self) -> &String {
        &self.name
    }
}

#[derive(Debug)]
struct Company<'a> {
    name: String,
    ceo: &'a Person
}

impl<'a> std::fmt::Display for Company<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Company: {} - CEO: {}", self.name, self.ceo.name)
    }
}

struct Person2<'a> {
    name: &'a str,
}

impl<'a> Person2<'a> {
    fn new(name: &'a str) -> Self {
        Person2 { name }
    }
    fn talk(&self) {
        println!("Hi, my name is {}.", self.name);
    }
}

fn main() {
    // let mut s: &String;
    // {
    //     let boss = Person { name: String::from("Elon Musk") };
    //     s = boss.ref_name();
    // }
    // println!("{}", s);

    let boss = Person { name: String::from("Elon Musk") };
    let tesla = Company { name: String::from("Tesla"), ceo: &boss };
    println!("{:?}", tesla);
    println!("{}", tesla);
    println!("{}", boss.ref_name());

    let p2 = Person2::new("Clayton");
    p2.talk();
}
