const MEANING_OF_LIFE: u8 = 42;
static A: i32 = 123;
static mut B: i32 = 456;

fn main() {
    println!("{}", MEANING_OF_LIFE);
    println!("A = {}", A);
    // println!("B = {}", B); // Erro de segurança (race condition)

    unsafe {
        println!("B = {}", B);
        B = 123;
        println!("B = {}", B);
    }
}
