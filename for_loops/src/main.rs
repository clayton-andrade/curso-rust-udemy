fn main() {
    for i in 1..=10 {
        println!("i = {}", i);
    }

    for (i, e) in (10..=20).enumerate() {
        println!("elemento {}, posição {}", e, i);
    }
}
