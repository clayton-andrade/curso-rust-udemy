fn main() {
    let a = vec![1, 2, 3, 4, 5];
    let mut b = Vec::<i32>::new();
    let mut c = Vec::new();
    c.push('c');
    c.push('l');
    b.push(8);
    b.push(8);
    let d = (0..10).collect::<Vec<u8>>();
    b[0] = 9;

    println!("a = {:?}, b = {:?}, c = {:?}, d = {:?}", a, b, c, d);

    // let elem = a[5];
    // println!("{}", elem);

    let elem = match c.get(2) {
        Some(e) => *e,
        None => ' ',
    };
    println!("{}", elem);

    if let Some(e) = a.get(6) {
        println!("{}", e);
    } else {
        println!("no such element");
    }

    let elem = b.get(1).unwrap_or(&-1);
    println!("{}", elem);

    let nome = "clayton".chars().collect::<Vec<char>>();
    for c in &nome {
        println!("{}", c);
    }
    println!("{:?}", nome);

    let mut a = vec![10, 20, 30, 40, 50, 60];
    let last_elem = a.pop();
    println!("Last element is {:?}, a = {:?}", last_elem, a);

    while let Some(e) = a.pop() {
        println!("{}", e);
    }

    let last_elem = a.pop();
    println!("Last element is {:?}, a = {:?}", last_elem, a);
}
