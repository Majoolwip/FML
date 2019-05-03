#[macro_use]
extern crate cfg_if;

cfg_if! {
    if #[cfg(feature="double")] {
        pub type Real = f64;
        pub const EPSILON: Real = 0.00001;
    } else {
        pub type Real = f32;
        pub const EPSILON: Real = 0.001;
    }
}

pub mod mat4;
pub mod quat;
pub mod shapes;
pub mod vecs;

#[derive(Debug, Clone, Copy, Default)]
pub struct Vec3 {
    pub x: Real,
    pub y: Real,
    pub z: Real,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Vec2 {
    pub x: Real,
    pub y: Real,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Quat {
    pub x: Real,
    pub y: Real,
    pub z: Real,
    pub w: Real,
}

#[derive(Debug, Clone, Copy)]
pub struct Mat4 {
    pub m: [Real; 16],
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}
