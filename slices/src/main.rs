fn main() {
    let mut data = [1, 2, 3, 4, 5];
    use_slice(&mut data[1..4]);
    println!("{:?}", data);
    use_slice(&mut data);
    println!("{:?}", data);
}

fn use_slice(slice: &mut[i32]) {
    println!("{:?}", slice);
    slice[0] = 2222;
}
