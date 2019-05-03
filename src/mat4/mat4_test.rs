use crate::Mat4;
use crate::Real;

#[test]
fn test_identity() {
    let m = Mat4::identity();
    let expected = Mat4 {
        m: [
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ],
    };
    assert_eq!(m, expected);
}

#[test]
fn test_translation() {
    let m = Mat4::translation(1.0, 2.0, 3.0);
    let expected = Mat4 {
        m: [
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 2.0, 3.0, 1.0,
        ],
    };
    assert_eq!(m, expected);
}

#[test]
fn test_scale() {
    let m = Mat4::scale(2.0, 3.0, 4.0);
    let expected = Mat4 {
        m: [
            2.0, 0.0, 0.0, 0.0, 0.0, 3.0, 0.0, 0.0, 0.0, 0.0, 4.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ],
    };
    assert_eq!(m, expected);
}

#[test]
fn test_rotation() {
    let m = Mat4::rotation(0.0, std::f32::consts::FRAC_PI_2 as Real, 0.0);
    let expected = Mat4 {
        m: [
            0.0, 0.0, -1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ],
    };
    assert_eq!(m, expected);
    let m = Mat4::rotation(std::f32::consts::FRAC_PI_2 as Real, 0.0, 0.0);
    let expected = Mat4 {
        m: [
            1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ],
    };
    assert_eq!(m, expected);
    let m = Mat4::rotation(0.0, 0.0, std::f32::consts::FRAC_PI_2 as Real);
    let expected = Mat4 {
        m: [
            0.0, 1.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ],
    };
    assert_eq!(m, expected);
}

#[test]
fn test_perspective() {
    let m = Mat4::perspective(45.0, 1280.0, 720.0, 0.1, 1000.0);
    let expected = Mat4 {
        m: [
            1.0 / ((45.0 / 2.0 as Real).tan() * (1280.0 / 720.0)),
            0.0,
            0.0,
            0.0,
            0.0,
            1.0 / ((45.0 / 2.0 as Real).tan()),
            0.0,
            0.0,
            0.0,
            0.0,
            1000.1 / (0.1 - 1000.0),
            -1.0,
            0.0,
            0.0,
            (2.0 * 0.1 * 1000.0) / (0.1 - 1000.0),
            0.0,
        ],
    };
    assert_eq!(m, expected);
}
