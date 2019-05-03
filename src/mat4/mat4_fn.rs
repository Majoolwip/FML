use crate::Mat4;
use crate::Real;
use crate::Vec3;

impl Mat4 {
    #[inline(always)]
    pub fn identity() -> Mat4 {
        Mat4 {
            m: [
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    #[inline(always)]
    pub fn translation(x: Real, y: Real, z: Real) -> Mat4 {
        Mat4 {
            m: [
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, x, y, z, 1.0,
            ],
        }
    }

    #[inline(always)]
    pub fn scale(x: Real, y: Real, z: Real) -> Mat4 {
        Mat4 {
            m: [
                x, 0.0, 0.0, 0.0, 0.0, y, 0.0, 0.0, 0.0, 0.0, z, 0.0, 0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    #[inline]
    pub fn rotation(x: Real, y: Real, z: Real) -> Mat4 {
        let cos_x = x.cos();
        let sin_x = x.sin();
        let cos_y = y.cos();
        let sin_y = y.sin();
        let cos_z = z.cos();
        let sin_z = z.sin();
        Mat4 {
            m: [
                cos_z * cos_y,
                sin_z * cos_y,
                -sin_y,
                0.0,
                cos_z * sin_y * sin_x + -sin_z * cos_x,
                sin_z * sin_y * sin_x + cos_z * cos_x,
                cos_y * sin_x,
                0.0,
                cos_z * sin_y * cos_x + -sin_z * -sin_x,
                sin_z * sin_y * cos_x + cos_z * -sin_x,
                cos_y * cos_x,
                0.0,
                0.0,
                0.0,
                0.0,
                1.0,
            ],
        }
    }

    #[inline]
    pub fn perspective(v_fov: Real, width: Real, height: Real, near: Real, far: Real) -> Mat4 {
        let half_fov = (v_fov / 2.0).tan();
        let range = near - far;
        let aspect = width / height;
        Mat4 {
            m: [
                1.0 / (half_fov * aspect),
                0.0,
                0.0,
                0.0,
                0.0,
                1.0 / half_fov,
                0.0,
                0.0,
                0.0,
                0.0,
                (far + near) / range,
                -1.0,
                0.0,
                0.0,
                2.0 * far * near / range,
                0.0,
            ],
        }
    }

    #[inline]
    pub fn orthographic(
        left: Real,
        right: Real,
        top: Real,
        bottom: Real,
        near: Real,
        far: Real,
    ) -> Mat4 {
        let width = right - left;
        let height = top - bottom;
        let depth = far - near;
        Mat4 {
            m: [
                2.0 / width,
                0.0,
                0.0,
                0.0,
                0.0,
                2.0 / height,
                0.0,
                0.0,
                0.0,
                0.0,
                -2.0 / depth,
                0.0,
                -(right + left) / width,
                -(top + bottom) / height,
                -(far + near) / depth,
                1.0,
            ],
        }
    }

    #[inline(always)]
    pub fn transform(self, vec: Vec3) -> Vec3 {
        Vec3 {
            x: self.m[0] * vec.x + self.m[4] * vec.y + self.m[8] * vec.z + self.m[12],
            y: self.m[1] * vec.x + self.m[5] * vec.y + self.m[9] * vec.z + self.m[13],
            z: self.m[2] * vec.x + self.m[6] * vec.y + self.m[10] * vec.z + self.m[14],
        }
    }
}
