use std::ops::Mul;
use std::vec::Vec;
use std::iter::Sum;

pub fn dot_product<T>(a: Vec<T>, b: Vec<T>) -> T
where T: Sum + Copy + Mul<Output = T>
{
    return a.iter().zip(b.iter()).map(|(&x, &y)| x*y).sum();
}

pub struct Vector<T> {
    value: Vec<T>
}

impl<T> Vector<T> {
    pub fn new<U: Into<Vec<T>>>(vec: U) -> Vector<T> {
        Vector {
            value: vec.into(),
        }
    }
}

impl<T: Sum + Copy + Mul<Output = T>> Mul<Vector<T>> for Vector<T> {
    type Output = T;
    fn mul(self, rhs: Self) -> T {
        return dot_product(self.value, rhs.value);
    }
}
