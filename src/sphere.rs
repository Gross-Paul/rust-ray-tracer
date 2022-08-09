use crate::{vec3::Point3, material::Material};

pub struct Sphere {
    center: Point3,
    radius: f64,
    material : Box<dyn Material>
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material : Box<dyn Material>) -> Sphere {
        Sphere { center, radius, material }
    }

    pub fn center(&self) -> &Point3 {
        &self.center
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn material(&self) -> &Box<dyn Material> {
        &self.material
    }
}
