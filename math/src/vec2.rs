//! Code Ref: https://github.com/shaoruu/voxelize/blob/master/server/libs/vec.rs

use num::{cast, Float, Num};

use std::ops::{self, Index, IndexMut};

#[derive(Debug, Eq, PartialEq, Clone, Default, Hash)]
pub struct Vec2<T>(pub T, pub T);

impl<T: Num + Copy> Vec2<T> {
    /// Add self to another `Vec3`.
    pub fn add(&self, other: &Self) -> Self {
        Vec2(self.0 + other.0, self.1 + other.1)
    }

    /// Subtract self by another `Vec3`.
    pub fn sub(&self, other: &Self) -> Self {
        Vec2(self.0 - other.0, self.1 - other.1)
    }

    /// Copy anther `Vec3`'s content to self.
    pub fn copy(&mut self, other: &Self) -> &Self {
        self.0 = other.0;
        self.1 = other.1;
        self
    }

    /// Set the data of this `Vec3`.
    pub fn set(&mut self, x: T, y: T) -> &Self {
        self.0 = x;
        self.1 = y;
        self
    }

    /// Scale all elements of self.
    pub fn scale(&self, scale: T) -> Self {
        Vec2(self.0 * scale, self.1 * scale)
    }

    /// Add another scaled instance to self.
    pub fn scale_and_add(&self, other: &Self, scale: T) -> Self {
        Vec2(
            self.0 + other.0 * scale,
            self.1 + other.1 * scale,
        )
    }

    /// Instantiate a `Vec3` instance from a 3-element array.
    pub fn from_arr(arr: [T; 3]) -> Self {
        Vec2(arr[0], arr[1])
    }
}

impl<T: Float> Vec2<T> {
    /// Length of the vector.
    pub fn len(&self) -> T {
        (self.0 * self.0 + self.1 * self.1).sqrt()
    }

    /// Get the maximum element of two vectors.
    pub fn max(&self, other: &Self) -> Self {
        Vec2(
            Float::max(self.0, other.0),
            Float::max(self.1, other.1),
        )
    }

    /// Get the minimum element of two vectors.
    pub fn min(&self, other: &Self) -> Self {
        Vec2(
            Float::min(self.0, other.0),
            Float::min(self.1, other.1),
        )
    }
}

impl Vec2<f32> {
    pub fn normalize(&self) -> Self {
        let Self(x, y) = self;
        let len = x * x + y * y;
        if len > 0.0 {
            let len = 1.0 / len.sqrt();
            return Self(self.0 * len, self.1 * len);
        }
        self.to_owned()
    }
}

impl<T: Copy + 'static> Vec2<T> {
    pub fn from<U: cast::AsPrimitive<T>>(other: &Vec2<U>) -> Vec2<T> {
        Vec2(other.0.as_(), other.1.as_())
    }
}

impl<T: Copy> Vec2<T> {
    pub fn to_arr(&self) -> [T; 2] {
        [self.0, self.1]
    }
}

impl<T: Copy + 'static, U: cast::AsPrimitive<T>> From<&Vec2<U>> for Vec2<T> {
    fn from(other: &Vec2<U>) -> Self {
        Vec2(other.0.as_(), other.1.as_())
    }
}

impl<T: Copy + 'static, U: cast::AsPrimitive<T>> From<&[U; 2]> for Vec2<T> {
    fn from(other: &[U; 2]) -> Self {
        Vec2(other[0].as_(), other[1].as_())
    }
}

impl<T: Copy + 'static, U: cast::AsPrimitive<T>> Into<[T; 2]> for Vec2<U> {
    fn into(self) -> [T; 2] {
        [self.0.as_(), self.1.as_()]
    }
}

impl<T: Num + Copy + Default> ops::Add<&Vec2<T>> for &Vec2<T> {
    type Output = Vec2<T>;

    fn add(self, rhs: &Vec2<T>) -> Self::Output {
        let mut result = Vec2::default();
        result.0 = self.0 + rhs.0;
        result.1 = self.1 + rhs.1;
        result
    }
}

impl<T: Num + Copy + Default> ops::Sub<&Vec2<T>> for &Vec2<T> {
    type Output = Vec2<T>;

    fn sub(self, rhs: &Vec2<T>) -> Self::Output {
        let mut result = Vec2::default();
        result.0 = self.0 - rhs.0;
        result.1 = self.1 - rhs.1;
        result
    }
}

impl<T: Num + Copy + Default> ops::Mul<&Vec2<T>> for &Vec2<T> {
    type Output = Vec2<T>;

    fn mul(self, rhs: &Vec2<T>) -> Self::Output {
        let mut result = Vec2::default();
        result.0 = self.0 * rhs.0;
        result.1 = self.1 * rhs.1;
        result
    }
}

impl<T: Num + Copy + Default> ops::Mul<T> for &Vec2<T> {
    type Output = Vec2<T>;

    fn mul(self, rhs: T) -> Self::Output {
        let mut result = Vec2::default();
        result.0 = self.0 * rhs;
        result.1 = self.1 * rhs;
        result
    }
}

impl<T: Num + Copy + Default + ops::AddAssign> ops::AddAssign<Vec2<T>> for Vec2<T> {
    fn add_assign(&mut self, rhs: Vec2<T>) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl<T: Num + Copy + Default + ops::AddAssign> ops::AddAssign<[T; 2]> for Vec2<T> {
    fn add_assign(&mut self, rhs: [T; 2]) {
        self.0 += rhs[0];
        self.1 += rhs[1];
    }
}

impl<T: Num + Copy + Default + ops::SubAssign> ops::SubAssign<Vec2<T>> for Vec2<T> {
    fn sub_assign(&mut self, rhs: Vec2<T>) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}

impl<T: Num + Copy + Default + ops::MulAssign> ops::MulAssign<Vec2<T>> for Vec2<T> {
    fn mul_assign(&mut self, rhs: Vec2<T>) {
        self.0 *= rhs.0;
        self.1 *= rhs.1;
    }
}

impl<T: Num + Copy + Default + ops::MulAssign> ops::MulAssign<T> for Vec2<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.0 *= rhs;
        self.1 *= rhs;
    }
}

impl<T: Num + Copy + Default + ops::DivAssign> ops::DivAssign<T> for Vec2<T> {
    fn div_assign(&mut self, rhs: T) {
        self.0 /= rhs;
        self.1 /= rhs;
    }
}

impl<T: Num + Clone> Index<usize> for Vec2<T> {
    type Output = T;

    /// Index for accessing elements of this vector.
    fn index(&self, index: usize) -> &Self::Output {
        if index == 0 {
            &self.0
        } else if index == 1 {
            &self.1
        } else {
            panic!("Index out of bounds for accessing Vec2.");
        }
    }
}

impl<T: Num + Clone> IndexMut<usize> for Vec2<T> {
    /// Index for accessing mutable elements of this vector.
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index == 0 {
            &mut self.0
        } else if index == 1 {
            &mut self.1
        } else {
            panic!("Index out of bounds for accessing Vec2.");
        }
    }
}

impl<T: Num + Clone> From<Vec<T>> for Vec2<T> {
    /// Construct a `Vec3` instance from a primitive vector.
    fn from(vec: Vec<T>) -> Self {
        let x = vec[0].clone();
        let y = vec[1].clone();

        Self(x, y)
    }
}