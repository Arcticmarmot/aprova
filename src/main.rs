use std::ops::Add;
use std::ops::Neg;
use std::ops::AddAssign;
use std::cmp::{Ordering, PartialOrd};
use std::cmp::Reverse;
#[derive(Clone, Copy, Debug, PartialEq)]
struct Complex<T> {
    re: T,
    im: T,
}

impl<T> Add for Complex<T>
where
    T: Add<Output=T>
{
    type Output = Complex<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im
        }
    }
}

impl<L, R> Add<Complex<R>> for Complex<L>
where
    L: Add<R>
{
    type Output = Complex<L::Output>;

    fn add(self, rhs: Complex<R>) -> Self::Output {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im
        }
    }
}

impl<T> Neg for Complex<T>
where
    T: Neg<Output = T>
{
    type Output = Complex<T>;
    fn neg(self) -> Self::Output {
        Complex {
            re: -self.re,
            im: -self.im
        }
    }
}

impl<T> AddAssign for Complex<T>
where
    T: AddAssign<T>
{
    fn add_assign(&mut self, rhs: Self) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

impl<T: PartialEq> PartialEq for Complex<T> {
    fn eq(&self, other: &Self) -> bool {
        self.re == other.re && self.im == other.im
    }
}

#[derive(Debug, PartialEq)]
struct Interval<T> {
    lower: T,
    upper: T
}

impl<T: PartialOrd> PartialOrd<Interval<T>> for Interval<T> {
    fn partial_cmp(&self, other: &Interval<T>) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else if self.lower >= other.upper {
            Some(Ordering::Greater)
        } else if self.upper <= other.lower {
            Some(Ordering::Less)
        } else {
            None
        }
    }
}

use std::collections::HashMap;

struct Image<P> {
    width: usize,
    pixels: Vec<P>
}

impl<P: Default + Copy> Image<P> {
    fn new(width: usize, height: usize) -> Image<P> {
        Image {
            width,
            pixels: vec![P::default(); width * height]
        }
    }
}

use std::ops::{Index, IndexMut};
impl<P> Index<usize> for Image<P> {
    type Output = [P];
    fn index(&self, row: usize) -> &Self::Output {
        let start = row * self.width;
        &self.pixels[start..start + self.width]
    }
}

impl<P> IndexMut<usize> for Image<P> {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        let start = row * self.width;
        &mut self.pixels[start..start + self.width]
    }
}

fn main() {
    let c1 = Complex::<u32> { re: 12, im: 12 };
    let c2 = Complex::<u32> { re: 32, im: 1 };
    println!("{:?}", c1 + c2);
    println!("{:?}", c1);
    let c1 = Complex::<f32>{re: 1.0, im: 0.0/0.0 };
    let c2 = Complex::<f32>{re: 1.0, im: 0.0/0.0 };
    println!("{}", c1 == c1);

    assert!(Interval { lower: 10, upper: 20 } < Interval { lower: 20, upper: 40 });
    let left = Interval { lower: 10, upper: 30 };
    let right = Interval { lower: 20, upper: 40 };
    assert!(!(left < right));
    assert_eq!(left > right, false);


}
