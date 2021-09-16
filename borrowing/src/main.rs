fn main() {
    let print_vector = |v| println!("{:?}", v);

    let v = vec![1, 2, 3];
    print_vector(&v);
    println!("{}", &v[1]);

    let mut a = 40;
    {
        let b = &mut a;
        *b += 2;
    }
    println!("a = {}", a);

    let mut v = vec![1, 2, 3];
    for i in v.iter_mut() {
        println!("{}", i);
        // v.push(4);
    }
}
