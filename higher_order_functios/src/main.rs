fn main() {
    let limit = 500;
    let mut sum = 0;
    //let above_limit = |x| x > limit;
    let above_limit = greater_than(limit);

    for i in 0.. {
        let isq = i * i;
        if above_limit(isq) {
            break;
        }
        if is_even(isq) {
            sum += isq;
        }
    }

    println!("sum = {}", sum);

    let sum2 = (0..)
        .map(|x| x * x)
        .take_while(|&x| x < limit)
        .filter(|&x| is_even(x))
        .fold(0, |acc, x| acc + x);

    println!("sum = {}", sum2);
}

fn is_even(num: i32) -> bool {
    num % 2 == 0
}

fn greater_than(limit: i32) -> impl Fn(i32) -> bool {
    move |x| x > limit
}