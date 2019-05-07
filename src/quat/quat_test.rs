use crate::Quat;
use crate::Vec3;
use crate::Real;
use crate::EPSILON;

#[test]
fn from_axis() {
    let a = Quat::from_axis(&Vec3::new(0.0,1.0,0.0), 0.0);
    assert!(a.x == 0.0);
    assert!(a.y == 0.0);
    assert!(a.z == 0.0);
    assert!(a.w == 1.0);
}

#[test]
fn len() {
    let a = Quat::new(1.0, 2.0, 3.0, 4.0);
    let len = a.len();
    assert_eq!(len, ((1.0 * 1.0 + 2.0 * 2.0 + 3.0 * 3.0 + 4.0 * 4.0) as Real).sqrt());
}

#[test]
fn normalized() {
    let a = Quat::new(1.0, 2.0, 3.0, 4.0).normalized();
    let len = a.len();
    assert!((1.0 - len).abs() < EPSILON);
}

#[test]
fn conjugate() {
    let a = Quat::new(1.0, 2.0, 3.0, 4.0).conjugate();
    assert_eq!(a.x, -1.0);
    assert_eq!(a.y, -2.0);
    assert_eq!(a.z, -3.0);
    assert_eq!(a.w, 4.0);
}

#[test]
fn dot() {
    let a = Quat::new(1.0, 2.0, 3.0, 4.0);
    let dot = a.dot(&a);
    assert_eq!(dot, (1.0 * 1.0 + 2.0 * 2.0 + 3.0 * 3.0 + 4.0 * 4.0) as Real);
}

#[test]
fn nlerp() {
    let a = Quat::from_axis(&Vec3::new(0.0, 1.0, 0.0), 0.0);
    let b = Quat::from_axis(&Vec3::new(1.0, 0.0, 0.0), crate::REAL_FRAC_PI_2);
    let test = a.nlerp(b, 0.0);
    assert_eq!(a, test);
    let test = a.nlerp(b, 1.0);
    assert_eq!(b, test);
}

#[test]
fn slerp() {
    let a = Quat::from_axis(&Vec3::new(0.0, 1.0, 0.0), 0.0);
    let b = Quat::from_axis(&Vec3::new(1.0, 0.0, 0.0), crate::REAL_FRAC_PI_2);
    let test = a.slerp(b, 0.0);
    assert_eq!(a, test);
    let test = a.slerp(b, 1.0);
    assert_eq!(b, test);
}

#[test]
fn forward() {
    let a = Quat::from_axis(&Vec3::new(0.0, 1.0, 0.0), 0.0);
    let forward = a.get_forward();
    assert_eq!(forward, Vec3::new(0.0, 0.0, 1.0));
}

#[test]
fn up() {
    let a = Quat::from_axis(&Vec3::new(0.0, 1.0, 0.0), 0.0);
    let forward = a.get_up();
    assert_eq!(forward, Vec3::new(0.0, 1.0, 0.0));
}

#[test]
fn right() {
    let a = Quat::from_axis(&Vec3::new(0.0, 1.0, 0.0), 0.0);
    let forward = a.get_right();
    assert_eq!(forward, Vec3::new(1.0, 0.0, 0.0));
}

#[test]
fn add() {
    let a = Quat::new(1.0, 2.0, 3.0, 4.0);
    let b = Quat::new(5.0, 6.0, 7.0, 8.0);
    let c = a + b;
    assert_eq!(c, Quat::new(6.0, 8.0, 10.0, 12.0));
}

#[test]
fn sub() {
    let a = Quat::new(10.0, 20.0, 30.0, 40.0);
    let b = Quat::new(5.0, 6.0, 7.0, 8.0);
    let c = a - b;
    assert_eq!(c, Quat::new(5.0, 14.0, 23.0, 32.0));
}


