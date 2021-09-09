fn main() {
    let sh = || println!("Hello, closure!");
    sh();
    say_hello();

    let plus_one = |x| x + 1;
    let n = 5;
    println!("{} + 1 = {}", n, plus_one(n));

    let two = 2;
    let plus_two = |&x| {
        let mut z = x;
        z += two;
        z
    };    
    println!("{} + 2 = {}", 8, plus_two(&8));
    let borrow_two = &two;
    println!("{}", borrow_two);
}

fn say_hello() {
    println!("Hello, function!");
}
