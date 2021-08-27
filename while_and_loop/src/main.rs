fn main() {
    let mut x = 1;
    while x < 1000 {
        x *= 2;
        println!("x = {}", x);
    }

    let mut x = 100;
    while x >= 0 {
        if x % 10 == 0 {
            x -= 1;
            continue;
        }
        println!("x = {}", x);
        x -= 1;
    }

    let mut x = 1;
    loop {
        x *= 2;
        println!("x = {}", x);
        if x == 1 << 10 { // 2^10 = 1024
            break;
        }
    }
}
