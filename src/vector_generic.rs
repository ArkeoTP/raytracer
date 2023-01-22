use std::ops::{Add, Div, Mul, Sub};

pub trait Scalar: Add + Sub + Mul + Div + Sized + Copy {}

trait Sqrt {
    fn sqrt(self) -> Self;
}

impl Sqrt for f64 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}

impl Sqrt for f32 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3<T>(T, T, T)
where
    T: Scalar;

impl<T: Scalar + Mul<Output = T> + Add<Output = T>> Vec3<T>
where
    T: Scalar + Mul<Output = T> + Add<Output = T> + Sub<Output = T>,
{
    pub fn dot(self, other: Vec3<T>) -> T {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    pub fn cross(self, other: Vec3<T>) -> Vec3<T> {
        self
    }

    pub fn length_squared(self) -> T {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }
}

/// Add two vectors
impl<T: Scalar + Add<Output = T>> Add for Vec3<T> {
    type Output = Vec3<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

/// Subtract two vectors
impl<T: Scalar + Sub<Output = T>> Sub for Vec3<T> {
    type Output = Vec3<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

/// Multiply two vectors
impl<T: Scalar + Mul<Output = T>> Mul for Vec3<T> {
    type Output = Vec3<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

/// Divide two vectors
impl<T: Scalar + Div<Output = T>> Div for Vec3<T> {
    type Output = Vec3<T>;

    fn div(self, rhs: Self) -> Self::Output {
        Vec3(self.0 / rhs.0, self.1 / rhs.1, self.2 / rhs.2)
    }
}

pub type Point3 = Vec3<f64>;
pub type Color = Vec3<f64>;
