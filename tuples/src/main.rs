fn main() {
    let x = 3;
    let y = 5;

    let sp = sum_and_product(x, y);
    println!("sum = {}, product = {}", sp.0, sp.1);

    let (sum, product) = sum_and_product(x, y);
    println!("sum = {}, product = {}", sum, product);

    println!("sp = {:?}", sp);

    let combined = (sum_and_product(x, y), sum_and_product(4, 9));
    println!("combined = {:?}", combined);
    println!("last element = {}", combined.1.1);

    let ((a, b), (c, _)) = combined;
    println!("combined = ({}, {}, {})", a, b, c);

    let foo = (true, 22.5, -5i8);
    println!("foo = {:?}", foo);

    let meaning = (42,);
    println!("{:?}", meaning);
}

fn sum_and_product(a: i32, b: i32) -> (i32, i32) {
    (a + b, a * b)
}