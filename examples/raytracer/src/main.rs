use fml::shapes::Intersection;
use fml::shapes::Intrinsic;
use fml::{Ray, Vec3};
use image::{ImageBuffer, Rgb};

fn main() {
    const WIDTH: u32 = 1920;
    const HEIGHT: u32 = 1080;
    const WIDTH_F: f32 = 1920.0;
    const HEIGHT_F: f32 = 1080.0;
    const OUTPUT: &str = "output.png";

    let mut scene = Scene {
        objects: vec![],
        sun: Vec3::new(1.0, -1.0, -1.0).negate().normalized(),
    };

    scene.objects.push(Object {
        shape: Intrinsic::ISphere {
            origin: Vec3::new(0.0, 0.0, 0.0),
            radius: 1.0,
        },
        color: Vec3::new(1.0, 0.0, 0.0),
    });

    scene.objects.push(Object {
        shape: Intrinsic::IPlane {
            origin: Vec3::new(0.0, -2.0, 0.0),
            normal: Vec3::new(0.0, 1.0, 0.0),
        },
        color: Vec3::new(0.0, 1.0, 0.0),
    });

    scene.objects.push(Object {
        shape: Intrinsic::IPlane {
            origin: Vec3::new(0.0, -2.0, -3.0),
            normal: Vec3::new(-1.0, 0.0, 1.0).normalized(),
        },
        color: Vec3::new(0.0, 0.0, 1.0),
    });

    let mut img = ImageBuffer::new(WIDTH, HEIGHT);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let ray_x = (x as f32 / WIDTH_F) * 2.0 - 1.0;
        let ray_y = -((y as f32 / HEIGHT_F) * 2.0 - 1.0) * (HEIGHT_F / WIDTH_F);
        let ray = Ray {
            origin: Vec3::new(0.0, 0.0, 5.0),
            direction: Vec3::new(ray_x, ray_y, -1.0).normalized(),
        };
        let color = trace(&scene, ray, 0);
        *pixel = Rgb([
            (color.x * 255.0) as u8,
            (color.y * 255.0) as u8,
            (color.z * 255.0) as u8,
        ]);
    }

    img.save(OUTPUT).unwrap();
}

struct Scene {
    pub objects: Vec<Object>,
    pub sun: Vec3,
}

struct Object {
    pub shape: Intrinsic,
    pub color: Vec3,
}

fn trace(scene: &Scene, ray: Ray, mut depth: i32) -> Vec3 {
    if depth < 3 {
        match scene.closest_intersection(ray) {
            None => Vec3::new(0.0, 0.0, 0.0),
            Some((object, inter)) => {
                let amt = inter.normal.dot(scene.sun).max(0.0);
                let reflect = Ray {
                    origin: inter.position + (inter.normal * 0.01),
                    direction: ray.direction.reflect(inter.normal),
                };
                depth = depth + 1;
                let color = object.color * amt + (trace(&scene, reflect, depth) * 0.1);
                return color;
            }
        };
    }
    Vec3::new(0.0, 0.0, 0.0)
}

impl Scene {
    pub fn closest_intersection(&self, ray: Ray) -> Option<(&Object, Intersection)> {
        let mut min_intersection = None;
        let mut min_object = None;
        let mut min = std::f32::MAX;

        for object in &self.objects {
            match object.shape.get_intersection(ray) {
                None => (),
                Some(inter) => {
                    if min > inter.distance {
                        min = inter.distance;
                        min_intersection = Some(inter);
                        min_object = Some(object);
                    }
                }
            }
        }
        if min != std::f32::MAX {
            Some((min_object.unwrap(), min_intersection.unwrap()))
        } else {
            None
        }
    }
}
