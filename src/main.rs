use camera::Camera;
use consts::{IMAGE_HEIGHT, IMAGE_WIDTH};
use hit::{Hittable, HittableList};
use indicatif::ProgressIterator;
use material::{Dielectric, Lambertian, Metal};
use rand::{thread_rng, Rng};
use ray::Ray;
use sphere::Sphere;
use std::io::Write;
use std::thread;
use std::{f64::INFINITY, fs::File};
use vec3::{ColorF64, ColorU8, Point3};

mod camera;
mod consts;
mod hit;
mod material;
mod ray;
mod sphere;
mod vec3;

// Determine the color of a ray by scattering it in the scene.
fn get_ray_color(ray: &Ray, hittable_list: &HittableList, depth: i32) -> ColorF64 {
    if depth <= 0 {
        return ColorF64::ZERO;
    }

    // If Hit
    if let Some(hit_record) = hittable_list.hit(&ray, 0.0, INFINITY) {
        let mut scattered: Ray = Ray { ..*ray };
        let mut attenuation: ColorF64 = ColorF64::ONE;

        hit_record
            .material()
            .scatter(&ray, &hit_record, &mut attenuation, &mut scattered);

        //let reflected_ray = Ray::new(*hit_record.hitting_point() , *hit_record.normal() + Vec3::random_point_on_unit_sphere());
        return attenuation * get_ray_color(&scattered, &hittable_list, depth - 1);
    }

    // If Miss
    let unit_dir = ray.direction();
    let t = 0.5 * (unit_dir.y() + 1.0);
    return (1. - t) * ColorF64::new(1.0, 1.0, 1.0) + t * ColorF64::new(0.5, 0.7, 1.0);
}
fn main() {
    let mut file = File::create("out.ppm").expect("out.ppm can't be created");
    

    let max_depth = 10;

    let samples_per_pixel = 100;

    let camera = Camera::new();

    let mut content = format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT).to_owned();

    let sphere1 = Sphere::new(
        Point3::new(0.5, 0.0, -1.0),
        0.5,
        Box::new(Dielectric { index: 1.1 }),
    );
    let sphere2 = Sphere::new(
        Point3::new(-0.5, 0.0, -1.0),
        0.5,
        Box::new(Metal {
            albedo: ColorF64::new(1., 1., 1.),
        }),
    );
    let sphere3 = Sphere::new(
        Point3::new(-0.5, 0.0, 0.),
        0.5,
        Box::new(Metal {
            albedo: ColorF64::new(1., 1., 1.),
        }),
    );

    let mut hittable_list: Vec<Box<dyn Hittable>> =
        vec![Box::new(sphere1), Box::new(sphere2), Box::new(sphere3)];

    hittable_list.push(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        Box::new(Lambertian {
            albedo: ColorF64::new(0.5, 0.5, 0.5),
        }),
    )));

    let hittable_list = HittableList { hittable_list };

    for j in (0..IMAGE_HEIGHT - 1).rev().progress() {
        for i in 0..IMAGE_WIDTH {
            let mut ray_color = ColorF64::ZERO;

            let mut rng = thread_rng();

            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rng.gen::<f64>()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + rng.gen::<f64>()) / (IMAGE_HEIGHT - 1) as f64;

                let ray = camera.gen_ray(u, v);

                let sample_ray_color = get_ray_color(&ray, &hittable_list, max_depth);

                ray_color = ray_color + sample_ray_color / samples_per_pixel as f64;
            }

            let ray_color = ColorU8::from(ray_color);
            content.push_str(&String::from(ray_color));
        }
    }

    writeln!(&mut file, "{}", content).expect("Couldn't write inside file");
}
