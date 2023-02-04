use std::cmp::Ordering;
use std::ops::{Add, Div, Mul, Rem, Sub};

#[derive(Copy, Clone, Debug)]
pub struct ConstantValue(i32);

impl From<i32> for ConstantValue {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl From<ConstantValue> for i32 {
    fn from(value: ConstantValue) -> Self {
        value.0
    }
}

impl Add for ConstantValue {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl Sub for ConstantValue {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0)
    }
}

impl Mul for ConstantValue {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self(self.0 * other.0)
    }
}

impl Div for ConstantValue {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self(self.0 / other.0)
    }
}

impl Rem for ConstantValue {
    type Output = Self;

    fn rem(self, other: Self) -> Self {
        Self(self.0 % other.0)
    }
}

impl PartialOrd for ConstantValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl PartialEq for ConstantValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
