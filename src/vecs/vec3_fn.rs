use crate::Real;
use crate::Vec3;

impl Vec3 {
    #[inline(always)]
    pub fn new(x: Real, y: Real, z: Real) -> Vec3 {
        Vec3 { x, y, z }
    }

    #[inline(always)]
    pub fn from_slice(slice: [Real; 3]) -> Vec3 {
        Vec3 {
            x: slice[0],
            y: slice[1],
            z: slice[2],
        }
    }

    #[inline(always)]
    pub fn len_sqrd(&self) -> Real {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    #[inline(always)]
    pub fn len(&self) -> Real {
        self.len_sqrd().sqrt()
    }

    #[inline(always)]
    pub fn normalized(&self) -> Vec3 {
        let len = self.len();
        Vec3 {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }

    #[inline(always)]
    pub fn dot(&self, other: &Vec3) -> Real {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    #[inline(always)]
    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    #[inline(always)]
    pub fn reflect(&self, normal: &Vec3) -> Vec3 {
        let twice_dot = 2.0 * self.dot(normal);
        Vec3 {
            x: self.x - twice_dot * normal.x,
            y: self.y - twice_dot * normal.y,
            z: self.z - twice_dot * normal.z,
        }
    }

    #[inline(always)]
    pub fn lerp(&self, target: &Vec3, percentage: Real) -> Vec3 {
        Vec3 {
            x: self.x + percentage * (target.x - self.x),
            y: self.y + percentage * (target.y - self.y),
            z: self.z + percentage * (target.z - self.z),
        }
    }

    #[inline(always)]
    pub fn nlerp(&self, target: &Vec3, percentage: Real) -> Vec3 {
        self.lerp(target, percentage).normalized()
    }

    #[inline(always)]
    pub fn negate(&self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    #[inline(always)]
    pub fn inverse(&self) -> Vec3 {
        Vec3 {
            x: 1.0 / self.x,
            y: 1.0 / self.y,
            z: 1.0 / self.z,
        }
    }

    #[inline(always)]
    pub fn abs(&self) -> Vec3 {
        Vec3 {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }

    #[inline(always)]
    pub fn clamp(&self, min: Real, max: Real) -> Vec3 {
        Vec3 {
            x: self.x.min(max).max(min),
            y: self.y.min(max).max(min),
            z: self.z.min(max).max(min),
        }
    }
}
