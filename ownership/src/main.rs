#![allow(unused_variables)]

fn main() {
    let v = vec![1, 2, 3];
    let v2 = v;
    // println!("{:?}", v);
    println!("{:?}", v2);

    let foo = |v: Vec<i32>| ();
    foo(v2);
    // println!("{:?}", v2);

    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    let x = Box::new(5);
    let y = x;
    // println!("x = {}, y = {}", x, y);

    
}
