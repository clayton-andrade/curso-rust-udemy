fn main() {
    for i in 0..=12 {
        println!("{}: I have {} oranges", i, how_many(i).0);
    }

    let point = (0, 0);
    match point {
        (0, 0) => println!("origin"),
        (0, y) => println!("x axis, y = {}", y),
        (x, 0) => println!("y axis, x = {}", x),
        (x, y) => println!("({}, {})", x, y),
    }
}

fn how_many(x: u16) -> (&'static str, u16) {
    match x {
        0 => ("no", x),
        1 | 2 => ("one or two", x),
        z @ 9..=11 => {
            println!("z + 1 = {}", z);
            ("lots of", z)
        },
        12 => ("a dozen", x),
        y if x % 2 == 0 => {
            println!("y = {}", y);
            ("some", y)
        },
        _ => ("a few", x),
    }
}
