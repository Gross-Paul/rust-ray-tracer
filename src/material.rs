use crate::{
    hit::HitRecord,
    ray::Ray,
    vec3::{ColorF64, Vec3},
};

pub trait Material {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut ColorF64,
        scattered: &mut Ray,
    );
}

pub struct Lambertian {
    pub albedo: ColorF64,
}

pub struct Metal {
    pub albedo: ColorF64,
}

impl Material for Lambertian {
    fn scatter(
        &self,
        ray_in: &Ray,
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

impl Material for Metal {
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
