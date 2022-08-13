use rand::Rng;

use crate::{
    hit::HitRecord,
    ray::Ray,
    vec3::{ColorF64, Vec3},
};

pub trait Scatterable: Sync + Send {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut ColorF64,
        scattered: &mut Ray,
    );
}

pub struct Metal {
    pub albedo: ColorF64,
}

pub struct Lambertian {
    pub albedo: ColorF64,
}

pub struct Dielectric {
    pub index: f64,
}

impl Scatterable for Lambertian {
    fn scatter(
        &self,
        _ray_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut ColorF64,
        scattered: &mut Ray,
    ) {
        let scattered_ray = *hit_record.hitting_point()
            + *hit_record.normal()
            + Vec3::random_point_on_unit_sphere();
        *scattered = Ray::new(*hit_record.hitting_point(), scattered_ray);
        *attenuation = self.albedo;
    }
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - 2.0 * v.dot(n) * *n
}

impl Scatterable for Metal {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut ColorF64,
        scattered: &mut Ray,
    ) {
        let scattered_ray = reflect(&ray_in.direction().unit_vector(), hit_record.normal());
        *scattered = Ray::new(*hit_record.hitting_point(), scattered_ray);
        *attenuation = self.albedo;
    }
}

fn reflectance(cosine: f64, index: f64) -> f64 {
    let mut r0 = (1.0 - index) / (1.0 + index);
    r0 *= r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

impl Scatterable for Dielectric {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut ColorF64,
        scattered: &mut Ray,
    ) {
        *attenuation = Vec3::ONE;
        let refraction_ratio = if hit_record.front_face() {
            1.0 / self.index
        } else {
            self.index
        };

        let unit_direction = ray_in.direction().unit_vector();

        let cos_theta = f64::min((-unit_direction).dot(hit_record.normal()), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let direction: Vec3;

        if refraction_ratio * sin_theta > 1.0 || rand::thread_rng().gen::<f64>() < reflectance(cos_theta, refraction_ratio) {
            direction = reflect(&unit_direction, hit_record.normal());
        } else {
            let vector_out_perpendicular =
                refraction_ratio * (unit_direction + cos_theta * *hit_record.normal());
            let vector_out_parallel =
                -f64::sqrt(f64::abs(1.0 - vector_out_perpendicular.length_squared()))
                    * *hit_record.normal();

            direction = vector_out_perpendicular + vector_out_parallel;
        }

        *scattered = Ray::new(hit_record.hitting_point().clone(), direction);
    }
}
