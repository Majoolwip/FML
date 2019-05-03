use crate::Mat4;
use crate::Real;
use crate::EPSILON;
use std::ops::{Index, IndexMut, Mul, MulAssign};

impl Mul for Mat4 {
    type Output = Mat4;
    fn mul(self, other: Mat4) -> Mat4 {
        let mut res = [0.0; 16];
        for i in 0..4 {
            for j in 0..4 {
                res[i + j * 4] = self[i] * other[j * 4]
                    + self[i + 4] * other[1 + j * 4]
                    + self[i + 8] * other[2 + j * 4]
                    + self[i + 12] * other[3 + j * 4];
            }
        }
        Mat4 { m: res }
    }
}

impl MulAssign for Mat4 {
    fn mul_assign(&mut self, other: Mat4) {
        let mut res = [0.0; 16];
        for i in 0..4 {
            for j in 0..4 {
                res[i + j * 4] = self[i] * other[j * 4]
                    + self[i + 4] * other[1 + j * 4]
                    + self[i + 8] * other[2 + j * 4]
                    + self[i + 12] * other[3 + j * 4];
            }
        }
        *self = Mat4 { m: res };
    }
}

impl Index<usize> for Mat4 {
    type Output = Real;
    fn index(&self, index: usize) -> &Real {
        &self.m[index]
    }
}

impl IndexMut<usize> for Mat4 {
    fn index_mut(&mut self, index: usize) -> &mut Real {
        &mut self.m[index]
    }
}

impl PartialEq for Mat4 {
    fn eq(&self, other: &Mat4) -> bool {
        for (index, value) in self.m.into_iter().enumerate() {
            if (value - other[index]).abs() > EPSILON {
                return false;
            }
        }
        return true;
    }
}
