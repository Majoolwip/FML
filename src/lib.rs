#[macro_use]
extern crate cfg_if;

cfg_if! {
    if #[cfg(feature="double")] {
        pub type Real = f64;
        pub const EPSILON: Real = 1e-8;
        pub const REAL_PI: Real = std::f64::consts::PI;
        pub const REAL_FRAC_PI_2: Real  = std::f64::consts::FRAC_PI_2;
        pub const REAL_FRAC_PI_3: Real  = std::f64::consts::FRAC_PI_3;
        pub const REAL_FRAC_PI_4: Real  = std::f64::consts::FRAC_PI_4;
        pub const REAL_FRAC_PI_6: Real  = std::f64::consts::FRAC_PI_6;
        pub const REAL_FRAC_PI_8: Real  = std::f64::consts::FRAC_PI_8;
        pub const REAL_FRAC_1_PI: Real  = std::f64::consts::FRAC_1_PI;
        pub const REAL_FRAC_2_PI: Real  = std::f64::consts::FRAC_2_PI;
        pub const REAL_FRAC_2_SQRT_PI: Real  = std::f64::consts::FRAC_2_SQRT_PI;
        pub const REAL_FRAC_SQRT_2: Real  = std::f64::consts::SQRT_2;
        pub const REAL_FRAC_1_SQRT_2: Real  = std::f64::consts::FRAC_1_SQRT_2;
        pub const REAL_E: Real  = std::f64::consts::E;
        pub const REAL_LOG2_E: Real  = std::f64::consts::LOG2_E;
        pub const REAL_LOG10_E: Real  = std::f64::consts::LOG10_E;
        pub const REAL_LN_2: Real  = std::f64::consts::LN_2;
        pub const REAL_LN_10: Real  = std::f64::consts::LN_10;
    } else {
        pub type Real = f32;
        pub const EPSILON: Real = 1e-4;
        pub const REAL_PI: Real = std::f32::consts::PI;
        pub const REAL_FRAC_PI_2: Real  = std::f32::consts::FRAC_PI_2;
        pub const REAL_FRAC_PI_3: Real  = std::f32::consts::FRAC_PI_3;
        pub const REAL_FRAC_PI_4: Real  = std::f32::consts::FRAC_PI_4;
        pub const REAL_FRAC_PI_6: Real  = std::f32::consts::FRAC_PI_6;
        pub const REAL_FRAC_PI_8: Real  = std::f32::consts::FRAC_PI_8;
        pub const REAL_FRAC_1_PI: Real  = std::f32::consts::FRAC_1_PI;
        pub const REAL_FRAC_2_PI: Real  = std::f32::consts::FRAC_2_PI;
        pub const REAL_FRAC_2_SQRT_PI: Real  = std::f32::consts::FRAC_2_SQRT_PI;
        pub const REAL_FRAC_SQRT_2: Real  = std::f32::consts::SQRT_2;
        pub const REAL_FRAC_1_SQRT_2: Real  = std::f32::consts::FRAC_1_SQRT_2;
        pub const REAL_E: Real  = std::f32::consts::E;
        pub const REAL_LOG2_E: Real  = std::f32::consts::LOG2_E;
        pub const REAL_LOG10_E: Real  = std::f32::consts::LOG10_E;
        pub const REAL_LN_2: Real  = std::f32::consts::LN_2;
        pub const REAL_LN_10: Real  = std::f32::consts::LN_10;
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
