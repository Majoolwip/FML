use crate::Quat;
use crate::Vec3;
use crate::EPSILON;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

impl Add for Quat {
    type Output = Quat;
    fn add(self, other: Quat) -> Quat {
        Quat {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl AddAssign for Quat {
    fn add_assign(&mut self, other: Quat) {
        *self = Quat {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Sub for Quat {
    type Output = Quat;
    fn sub(self, other: Quat) -> Quat {
        Quat {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl SubAssign for Quat {
    fn sub_assign(&mut self, other: Quat) {
        *self = Quat {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}
impl Mul for Quat {
    type Output = Quat;
    fn mul(self, other: Quat) -> Quat {
        Quat {
            x: self.x * other.w + self.w * other.x + self.y * other.z - self.z * other.y,
            y: self.y * other.w + self.w * other.y + self.z * other.x - self.x * other.z,
            z: self.z * other.w + self.w * other.z + self.x * other.y - self.y * other.x,
            w: self.w * other.w - self.x * other.x - self.y * other.y - self.z * other.z,
        }
    }
}

impl MulAssign for Quat {
    fn mul_assign(&mut self, other: Quat) {
        *self = Quat {
            x: self.x * other.w + self.w * other.x + self.y * other.z - self.z * other.y,
            y: self.y * other.w + self.w * other.y + self.z * other.x - self.x * other.z,
            z: self.z * other.w + self.w * other.z + self.x * other.y - self.y * other.x,
            w: self.w * other.w - self.x * other.x - self.y * other.y - self.z * other.z,
        }
    }
}

impl Mul<Vec3> for Quat {
    type Output = Quat;
    fn mul(self, other: Vec3) -> Quat {
        Quat {
            x: self.w * other.x + self.y * other.z - self.z * other.y,
            y: self.w * other.y + self.z * other.x - self.x * other.x,
            z: self.w * other.z + self.x * other.y - self.y * other.x,
            w: -self.x * other.x - self.y * other.y - self.z * other.z,
        }
    }
}

impl Mul<Quat> for Vec3 {
    type Output = Quat;
    fn mul(self, other: Quat) -> Quat {
        Quat {
            x: other.w * self.x + other.y * self.z - other.z * self.y,
            y: other.w * self.y + other.z * self.x - other.x * self.x,
            z: other.w * self.z + other.x * self.y - other.y * self.x,
            w: -other.x * self.x - other.y * self.y - other.z * self.z,
        }
    }
}

impl MulAssign<Vec3> for Quat {
    fn mul_assign(&mut self, other: Vec3) {
        *self = Quat {
            x: self.w * other.x + self.y * other.z - self.z * other.y,
            y: self.w * other.y + self.z * other.x - self.x * other.x,
            z: self.w * other.z + self.x * other.y - self.y * other.x,
            w: -self.x * other.x - self.y * other.y - self.z * other.z,
        };
    }
}

impl PartialEq for Quat {
    fn eq(&self, other: &Quat) -> bool {
        (self.x - other.x).abs() < EPSILON
            && (self.y - other.y).abs() < EPSILON
            && (self.z - other.z).abs() < EPSILON
            && (self.w - other.w).abs() < EPSILON
    }
}
