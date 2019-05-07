use crate::Real;
use crate::Vec3;
use crate::EPSILON;

#[test]
fn new() {
    let a = Vec3::new(1.0, 2.0, 3.0);
    let b = Vec3 {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };
    assert_eq!(a, b);
}

#[test]
fn from_slice() {
    let a = Vec3::from_slice([1.0, 2.0, 3.0]);
    let b = Vec3 {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };
    assert_eq!(a, b);
}

#[test]
fn len_sqrd() {
    let a = Vec3::new(2.0, 3.0, 4.0);
    assert_eq!(a.len_sqrd(), 2.0 * 2.0 + 3.0 * 3.0 + 4.0 * 4.0);
    let a = Vec3::new(-2.0, -3.0, -4.0);
    assert_eq!(a.len_sqrd(), 2.0 * 2.0 + 3.0 * 3.0 + 4.0 * 4.0);
    let a = Vec3::new(0.0, 0.0, 0.0);
    assert_eq!(a.len_sqrd(), 0.0);
    let a = Vec3::new(0.0 / 0.0, 0.0 / 0.0, 0.0 / 0.0);
    assert!(a.len_sqrd().is_nan());
    let a = Vec3::new(1.0 / 0.0, 1.0 / 0.0, 1.0 / 0.0);
    assert!(a.len_sqrd().is_infinite());
}

#[test]
fn len() {
    let a = Vec3::new(2.0, 3.0, 4.0);
    assert_eq!(a.len(), (2.0 * 2.0 + 3.0 * 3.0 + 4.0 * 4.0 as Real).sqrt());
    let a = Vec3::new(-2.0, -3.0, -4.0);
    assert_eq!(a.len(), (2.0 * 2.0 + 3.0 * 3.0 + 4.0 * 4.0 as Real).sqrt());
    let a = Vec3::new(0.0, 0.0, 0.0);
    assert_eq!(a.len(), 0.0);
    let a = Vec3::new(0.0 / 0.0, 0.0 / 0.0, 0.0 / 0.0);
    assert!(a.len().is_nan());
    let a = Vec3::new(1.0 / 0.0, 1.0 / 0.0, 1.0 / 0.0);
    assert!(a.len().is_infinite());
}

#[test]
fn normalized() {
    let a = Vec3::new(2.0, 3.0, 4.0);
    assert!((a.normalized().len() - 1.0).abs() < EPSILON);
    let a = Vec3::new(-2.0, -3.0, -4.0);
    assert!((a.normalized().len() - 1.0).abs() < EPSILON);
    let a = Vec3::new(0.0, 0.0, 0.0);
    assert!(a.normalized().len().is_nan());
}

#[test]
fn dot() {
    let a = Vec3::new(2.0, 3.0, 4.0);
    let b = Vec3::new(-2.0, -3.0, -4.0);
    assert_eq!(a.dot(a), a.len_sqrd());
    assert_eq!(b.dot(b), b.len_sqrd());
    assert_eq!(a.dot(b), 2.0 * -2.0 + 3.0 * -3.0 + 4.0 * -4.0);
    assert_eq!(b.dot(a), 2.0 * -2.0 + 3.0 * -3.0 + 4.0 * -4.0);
    let c = Vec3::new(0.0, 0.0, 0.0);
    assert_eq!(a.dot(c), 0.0);
    assert_eq!(b.dot(c), 0.0);
    assert_eq!(c.dot(a), 0.0);
    assert_eq!(c.dot(b), 0.0);
    let d = Vec3::new(0.0 / 0.0, 0.0 / 0.0, 0.0 / 0.0);
    assert!(a.dot(d).is_nan());
    let d = Vec3::new(1.0 / 0.0, 1.0 / 0.0, 1.0 / 0.0);
    assert!(a.dot(d).is_infinite());
}

#[test]
fn cross() {
    let x = Vec3::new(1.0, 0.0, 0.0);
    let y = Vec3::new(0.0, 1.0, 0.0);
    let z = Vec3::new(0.0, 0.0, 1.0);
    assert_eq!(x.cross(y), z);
    assert_eq!(z.cross(x), y);
    assert_eq!(y.cross(z), x);
}

#[test]
fn reflect() {
    let norm = Vec3::new(0.0, 1.0, 0.0);
    let dir = Vec3::new(1.0, -1.0, 0.0);
    let reflect = dir.reflect(norm);
    assert_eq!(reflect, Vec3::new(1.0, 1.0, 0.0));
    let norm = Vec3::new(0.0, 0.0, 0.0);
    assert_eq!(dir.reflect(norm), dir);
}

#[test]
fn add() {
    let a = Vec3::new(1.0, 2.0, 3.0);
    let b = Vec3::new(2.0, 3.0, 4.0);
    let c = a + b;
    assert_eq!(c, Vec3::new(3.0, 5.0, 7.0));
}

#[test]
fn sub() {
    let a = Vec3::new(10.0, 20.0, 30.0);
    let b = Vec3::new(2.0, 3.0, 4.0);
    let c = a - b;
    assert_eq!(c, Vec3::new(8.0, 17.0, 26.0));
}

#[test]
fn mul() {
    let a = Vec3::new(10.0, 20.0, 30.0);
    let b = Vec3::new(2.0, 3.0, 4.0);
    let c = a * b;
    assert_eq!(c, Vec3::new(20.0, 60.0, 120.0));
    let c = a * 2.0;
    assert_eq!(c, Vec3::new(20.0, 40.0, 60.0));
    let c = 2.0 * a;
    assert_eq!(c, Vec3::new(20.0, 40.0, 60.0));

}

#[test]
fn div() {
    let a = Vec3::new(10.0, 20.0, 30.0);
    let b = Vec3::new(2.0, 3.0, 4.0);
    let c = a / b;
    assert_eq!(c, Vec3::new(5.0, 20.0 / 3.0, 30.0 / 4.0));
}
