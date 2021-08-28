fn main() {
    let x = 3;
    let y = 2;

    let result = if y != 0 { Some(x / y) } else { None };
    match result {
        Some(v) => println!("{} / {} = {}", x, y, v),
        None => println!("Cannot divide by zero"),
    }

    // if let Some(v) = result {
        // println!("{} / {} = {}", x, y, v);
    // } else {
        // println!("Cannot divide by zero");
    // }

    if let Some(v) = if y != 0 { Some(x / y) } else { None } {
        println!("{} / {} = {}", x, y, v);
    } else {
        println!("Cannot divide by zero");
    }

    let x = 10.0;
    let mut y = 10.0;
    let mut result = if y != 0.0 { Some(x / y) } else { None };

    while let Some(v) = result {
        println!("result = {}", v);
        y -= 1.0;
        result = if y != 0.0 { Some(x / y) } else { None };
    }
    println!("Cannot divide by zero");
}
