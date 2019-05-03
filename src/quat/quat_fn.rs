use crate::Quat;
use crate::Real;
use crate::Vec3;

impl Quat {
    #[inline(always)]
    pub fn new(x: Real, y: Real, z: Real, w: Real) -> Quat {
        Quat { x, y, z, w }
    }

    #[inline(always)]
    pub fn from_axis(axis: Vec3, radians: Real) -> Quat {
        let half_sin = (radians / 2.0).sin();
        let half_cos = (radians / 2.0).cos();
        Quat {
            x: axis.x * half_sin,
            y: axis.y * half_sin,
            z: axis.z * half_sin,
            w: half_cos,
        }
    }

    #[inline(always)]
    pub fn len(self) -> Real {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    #[inline(always)]
    pub fn normalized(self) -> Quat {
        let len = self.len();
        Quat {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
            w: self.w / len,
        }
    }

    #[inline(always)]
    pub fn conjugate(self) -> Quat {
        Quat {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: self.w,
        }
    }

    #[inline(always)]
    pub fn dot(self, other: Quat) -> Real {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    #[inline]
    pub fn nlerp(self, mut dest: Quat, percentage: Real) -> Quat {
        if self.dot(dest) < 0.0 {
            dest = Quat {
                x: -dest.x,
                y: -dest.y,
                z: -dest.z,
                w: -dest.w,
            }
        }
        Quat {
            x: self.x + percentage * (dest.x - self.x),
            y: self.y + percentage * (dest.y - self.y),
            z: self.z + percentage * (dest.z - self.z),
            w: self.w + percentage * (dest.w - self.w),
        }
        .normalized()
    }

    #[inline]
    pub fn slerp(self, mut dest: Quat, percentage: Real) -> Quat {
        let mut dot = self.dot(dest);
        if dot < 0.0 {
            dot = -dot;
            dest = Quat {
                x: -dest.x,
                y: -dest.y,
                z: -dest.z,
                w: -dest.w,
            }
        }

        if dot > 0.9995 {
            self.nlerp(dest, percentage)
        } else {
            let theta_0 = dot.acos();
            let theta = theta_0 * percentage;
            let sin_theta = theta.sin();
            let sin_theta_0 = theta_0.sin();
            let s0 = theta.cos() - dot * sin_theta / sin_theta_0;
            let s1 = sin_theta / sin_theta_0;
            Quat {
                x: self.x.mul_add(s0, dest.x * s1),
                y: self.y.mul_add(s0, dest.y * s1),
                z: self.z.mul_add(s0, dest.z * s1),
                w: self.w.mul_add(s0, dest.w * s1),
            }
            .normalized()
        }
    }

    #[inline(always)]
    pub fn get_forward(self) -> Vec3 {
        Vec3 {
            x: 2.0 * self.x * self.z - 2.0 * self.y * self.w,
            y: 2.0 * self.y * self.z + 2.0 * self.x * self.w,
            z: 1.0 - 2.0 * self.x * self.x - 2.0 * self.y * self.y,
        }
        .normalized()
    }

    #[inline(always)]
    pub fn get_up(self) -> Vec3 {
        Vec3 {
            x: 2.0 * self.x * self.y + 2.0 * self.z * self.w,
            y: 1.0 - 2.0 * self.x * self.x - 2.0 * self.z * self.z,
            z: 2.0 * self.y * self.z - 2.0 * self.x * self.w,
        }
    }

    #[inline(always)]
    pub fn get_right(self) -> Vec3 {
        Vec3 {
            x: 1.0 - 2.0 * self.y * self.y - 2.0 * self.z * self.z,
            y: 2.0 * self.x * self.y - 2.0 * self.z * self.w,
            z: 2.0 * self.x * self.z + 2.0 * self.y * self.w,
        }
    }
}
