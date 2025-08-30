use std::ops::Add;

#[derive(Clone, Copy, Debug)]
struct Complex<T: Add<Output=T> + Copy> {
    re: T,
    im: T,
}

impl<T: Add<Output=T> + Copy> Add for Complex<T> {
    type Output = Complex<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im
        }
    }
}

fn main() {
    let c1 = Complex::<u32> { re: 12, im: 12 };
    let c2 = Complex::<u32> { re: 32, im: 1 };
    println!("{:?}", c1 + c2);
    println!("{:?}", c1);
}
