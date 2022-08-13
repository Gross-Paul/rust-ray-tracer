use crate::{vec3, ray::Ray, consts::{VIEWPORT_WIDTH, VIEWPORT_HEIGHT, FOCAL_LENGTH}};
use vec3::{Point3, Vec3};


#[derive(Copy, Clone)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new() -> Camera {
        let horizontal = Point3::new(VIEWPORT_WIDTH, 0., 0.);
        let vertical = Point3::new(0., VIEWPORT_HEIGHT, 0.);
        let origin = Point3::ZERO;

        let lower_left_corner =
            origin - horizontal / 2. - vertical / 2. - Vec3::new(0., 0., FOCAL_LENGTH);

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn gen_ray(&self, u : f64, v : f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
