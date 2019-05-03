use crate::Real;
use crate::Vec2;

impl Vec2 {
    #[inline(always)]
    pub fn new(x: Real, y: Real) -> Vec2 {
        Vec2 { x, y }
    }

    #[inline(always)]
    pub fn len_sqrd(self) -> Real {
        self.x * self.x + self.y * self.y
    }

    #[inline(always)]
    pub fn len(self) -> Real {
        self.len_sqrd().sqrt()
    }

    #[inline(always)]
    pub fn normalized(self) -> Vec2 {
        let len = self.len();
        Vec2 {
            x: self.x / len,
            y: self.y / len,
        }
    }

    #[inline(always)]
    pub fn dot(self, other: Vec2) -> Real {
        self.x * other.x + self.y * other.y
    }

    #[inline(always)]
    pub fn clamp(self, min: Real, max: Real) -> Vec2 {
        Vec2 {
            x: self.x.min(max).max(min),
            y: self.y.min(max).max(min),
        }
    }

    #[inline(always)]
    pub fn inverse(self) -> Vec2 {
        Vec2 {
            x: 1.0 / self.x,
            y: 1.0 / self.y,
        }
    }

    #[inline(always)]
    pub fn abs(self) -> Vec2 {
        Vec2 {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }
}
