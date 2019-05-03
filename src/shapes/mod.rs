use crate::Ray;
use crate::Real;
use crate::Vec3;
use crate::EPSILON;

#[derive(Debug, Copy, Clone)]
pub enum Intrinsic {
    ISphere {
        origin: Vec3,
        radius: Real,
    },
    IPlane {
        origin: Vec3,
        normal: Vec3,
    },
    AABB {
        lb: Vec3,
        up: Vec3,
    },
    IDisk {
        origin: Vec3,
        normal: Vec3,
        radius: Real,
    },
}

pub struct Intersection {
    pub position: Vec3,
    pub normal: Vec3,
    pub distance: Real,
}

impl Intrinsic {
    pub fn get_intersection(&self, ray: Ray) -> Option<Intersection> {
        match self {
            Intrinsic::ISphere { origin, radius } => {
                let dist = ray.origin - *origin;
                let dd = ray.direction.dot(ray.direction);
                let discriminant =
                    (ray.direction.dot(dist)).powi(2) - dd * (dist.dot(dist) - radius.powi(2));
                let mut t;
                if discriminant < -EPSILON {
                    return None;
                } else if discriminant < EPSILON {
                    t = (ray.direction.negate().dot(dist) - discriminant.sqrt()) / dd;
                    if t < 0.0 {
                        return None;
                    }
                } else {
                    t = (ray.direction.negate().dot(dist) - discriminant.sqrt()) / dd;
                    if t < 0.0 {
                        t = (ray.direction.negate().dot(dist) + discriminant.sqrt()) / dd;
                        if t < 0.0 {
                            return None;
                        }
                    }
                }
                let position = ray.origin + ray.direction * t;
                Some(Intersection {
                    position,
                    normal: (position - *origin).normalized(),
                    distance: t,
                })
            }
            Intrinsic::IPlane { origin, normal } => {
                let dot = ray.direction.dot(*normal);
                if dot > -EPSILON {
                    None
                } else {
                    let t = (*origin - ray.origin).dot(*normal) / dot;
                    Some(Intersection {
                        position: ray.origin + t * ray.direction,
                        normal: *normal,
                        distance: t,
                    })
                }
            }
            _ => None,
        }
    }
}
