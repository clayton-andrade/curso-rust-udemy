fn main() {
    let mut s: &'static str = "hello, world";
    println!("{}", s);
    s = "hello";
    println!("{}", s);

    for c in s.chars() {
        println!("{}", c);
    }
    for c in s.chars().rev() {
        println!("{}", c);
    }

    if let Some(firts_char) = s.chars().nth(0) {
        println!("Frist char = '{}'", firts_char);
    }

    let mut letters = String::new();
    let mut letter = 'a' as u8;
    while letter <= 'z' as u8 {
        letters.push(letter as char);
        letter += 1;
    }
    println!("{}", letters);

    let name = "Clayton";
    let greeting = format!("Hi, I'm {}, nice to meet you.", name);
    println!("{}", greeting);

    let run = "run";
    let forest = "forest";
    let rfr = format!("{0}, {1}, {0}!", run, forest);
    println!("{}", rfr);

    println!("My name is {last}, {first} {last}.", first = "James", last = "Bond");
}
