fn main() {
    let mut a: [i32; 5] = [1, 2, 3, 4, 5];
    let b = [5, 4, 3, 2, 1];

    println!("a has {} elements, first is {}", a.len(), a[0]);
    println!("b has {} elements, first is {}", b.len(), b[0]);

    a[0] = 11;
    println!("a[0] = {}", a[0]);

    println!("a = {:?}", a);
    println!("b = {:?}", b);

    if a == [11, 2, 3, 4, 5] {
        println!("Match!");
    } else {
        println!("Does not match.");
    }

    if a == b {
        println!("Match!");
    } else {
        println!("Does not match.");
    }

    let c = [8u8; 10];
    for element in &c {
        println!("{}", element);
    }
    for (index, element) in c.iter().enumerate() {
        println!("c[{}]: {}", index, element);
    }
    for i in 0..c.len() {
        println!("c[{}]: {}", i, c[i]);
    }

    println!("a took up {} bytes", std::mem::size_of_val(&a));
    println!("c took up {} bytes", std::mem::size_of_val(&c));

    let m: [[u8; 3]; 3] = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    println!("m = {:?}", m);

    for row in &m {
        for col in row {
            print!("{} ", col);
        }
        println!()
    }

    println!();

    for i in 0..m.len() {
        for j in 0..m[i].len() {
            if i == j {
                print!("{} ", m[i][j]);
            } else {
                print!("x ");
            }
        }
        println!();
    }
}
