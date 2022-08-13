use crate::material::Scatterable;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::{Point3, Vec3};

pub struct HitRecord<'a> {
    hitting_point: Point3,
    normal: Vec3,
    t: f64,
    front_face: bool,
    material: &'a Box<dyn Scatterable>
}

pub struct HittableList {
    pub hittable_list: Vec<Box<dyn Hittable>>,
}

impl HitRecord<'_> {
    pub fn hitting_point(&self) -> &Point3 {
        &self.hitting_point
    }

    pub fn normal(&self) -> &Vec3 {
        &self.normal
    }

    pub fn material(&self) -> &Box<dyn Scatterable> {
        &self.material
    }

    pub fn front_face(&self) -> bool {
        self.front_face
    }

    fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = ray.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut optional_hit_record: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for hittable in self.hittable_list.iter() {
            let optional_hitting_point = hittable.hit(ray, t_min, closest_so_far);

            if let Some(hr) = optional_hitting_point {
                optional_hit_record = Some(HitRecord { ..hr });
                closest_so_far = hr.t;
            }
        }

        optional_hit_record
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = *ray.origin() - *self.center();

        let a = ray.direction().dot(ray.direction());
        let half_b = oc.dot(ray.direction());
        let c = oc.length_squared() - self.radius() * self.radius();

        let delta = half_b * half_b - a * c;
        let t = (-half_b - f64::sqrt(delta)) / a;

        if delta > 0.0 && t > t_min && t < t_max {
            let hitting_point = ray.at(t);
            let outward_normal = (hitting_point - *self.center()) / self.radius();

            let mut hit_record = HitRecord {
                hitting_point,
                normal: outward_normal,
                t,
                front_face: false,
                material: self.material()
            };
            hit_record.set_face_normal(ray, &outward_normal);
            return Some(hit_record);
        }

        None
    }
}
