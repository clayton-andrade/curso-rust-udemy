fn main() {
    print_value(45);
    let mut z = 1;
    increase(&mut z);
    print_value(z);
    let p = product(5, 7);
    print_value(p);
}

fn print_value(x: i32) {
    println!("value = {}", x);
}

fn increase(x: &mut i32) {
    *x += 1;
}

fn product(x: i32, y: i32) -> i32 {
    x * y
}
