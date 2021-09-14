use std::ops::{ Add, AddAssign, Neg };

#[derive(Debug, PartialEq, Eq)]
struct Complex<T> {
    re: T,
    im: T,
}

impl<T> Complex<T> {
    fn new(re: T, im: T) -> Self {
        Complex { re, im }
    }
}

impl<T> Add for Complex<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Complex { 
            re: self.re + other.re, 
            im: self.im + other.im,
        }
    }
}

impl<T> AddAssign for Complex<T>
where
    T: AddAssign
{
    fn add_assign(&mut self, other: Self) {
        self.re += other.re;
        self.im += other.im;
    }
}

impl<T> Neg for Complex<T>
where
    T: Neg<Output = T>
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Complex { re: -self.re, im: -self.im }
    }
}

// impl<T> PartialEq for Complex<T>
// where
//     T: PartialEq
// {
//     fn eq(&self, other: &Self) -> bool {
//         self.re == other.re && self.im == other.im
//     }
// }

fn main() {
    let a = Complex::new(1.2, 2.9);
    let b = Complex::new(3.3, 4.0);
    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", a + b);

    let mut c = Complex::new(5, 5);
    let d = Complex::new(5, 5);
    c += d;
    println!("{:?}", c);

    let e = Complex::new(2.77, 1.23);
    println!("{:?}", e);
    println!("{:?}", -e);

    let a = Complex::new(1.2, 2.9);
    let b = Complex::new(3.3, 4.0);
    let c = Complex::new(5, 5);
    let d = Complex::new(5, 5);
    println!("a = b: {}", a == b);
    println!("c = d: {}", c == d);
}
