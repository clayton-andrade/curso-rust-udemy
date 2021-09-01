fn main() {
    let mut vec = vec![1, 2, 3, 4, 5];

    for x in &vec { // Borrowing
        println!("{}", x);
    }
    println!("{:?}", vec);

    for x in vec.iter() { // Borrowing
        println!("{}", x);
    }
    println!("{:?}", vec);

    // for x in vec { // Move
    //     println!("{}", x);
    // }
    // println!("{:?}", vec);

    // for x in vec.into_iter() { // Move
    //     println!("{}", x);
    // }

    for x in vec.iter_mut() { // Borrowing Mutable
        *x *= 2;
        println!("{}", x);
    }
    println!("{:?}", vec);

    for x in vec.iter().rev() { // Borrowing
        println!("{}", x);
    }

    let vec = vec![1, 2, 3, 4, 5];
    let mut vec2 = Vec::<i32>::new();

    let mut it = vec.into_iter();
    while let Some(value) = it.next() {
        vec2.push(value * 10);
        print!("{} ", value);
    }
    println!();
    println!("{:?}", vec2);
}
