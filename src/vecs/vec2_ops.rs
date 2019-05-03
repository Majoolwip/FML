use crate::Real;
use crate::Vec2;
use crate::EPSILON;
use std::cmp::PartialEq;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, other: Vec2) -> Vec2 {
        Vec2::new(self.x + other.x, self.y + other.y)
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Vec2) {
        *self = Vec2::new(self.x + other.x, self.y + other.y);
    }
}

impl Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, other: Vec2) -> Vec2 {
        Vec2::new(self.x - other.x, self.y - other.y)
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, other: Vec2) {
        *self = Vec2::new(self.x - other.x, self.y - other.y);
    }
}

impl Mul for Vec2 {
    type Output = Vec2;
    fn mul(self, other: Vec2) -> Vec2 {
        Vec2::new(self.x * other.x, self.y * other.y)
    }
}

impl Mul<Real> for Vec2 {
    type Output = Vec2;
    fn mul(self, other: Real) -> Vec2 {
        Vec2::new(self.x * other, self.y * other)
    }
}

impl Mul<Vec2> for Real {
    type Output = Vec2;
    fn mul(self, other: Vec2) -> Vec2 {
        Vec2::new(self * other.x, self * other.y)
    }
}

impl MulAssign for Vec2 {
    fn mul_assign(&mut self, other: Vec2) {
        *self = Vec2::new(self.x * other.x, self.y * other.y);
    }
}

impl Div for Vec2 {
    type Output = Vec2;
    fn div(self, other: Vec2) -> Vec2 {
        Vec2::new(self.x / other.x, self.y / other.y)
    }
}

impl DivAssign for Vec2 {
    fn div_assign(&mut self, other: Vec2) {
        *self = Vec2::new(self.x / other.x, self.y / other.y);
    }
}

impl Index<usize> for Vec2 {
    type Output = Real;
    fn index(&self, index: usize) -> &Real {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Access of Vec2 index out of bounds!"),
        }
    }
}

impl IndexMut<usize> for Vec2 {
    fn index_mut(&mut self, index: usize) -> &mut Real {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Access of Vec2 index out of bounds!"),
        }
    }
}

impl Neg for Vec2 {
    type Output = Vec2;
    fn neg(self) -> Vec2 {
        Vec2::new(-self.x, -self.y)
    }
}

impl PartialEq for Vec2 {
    fn eq(&self, other: &Vec2) -> bool {
        (self.x - other.x).abs() < EPSILON && (self.y - other.y).abs() < EPSILON
    }
}
