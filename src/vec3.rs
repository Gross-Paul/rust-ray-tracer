use std::{ops::{Add, Deref, Div, Mul, Neg, Sub}, f64::consts::{FRAC_PI_2, PI}};

use rand::{thread_rng, Rng};

#[derive(Clone, Copy)]
pub struct Triple<T>(T, T, T);

impl<T: Copy> Triple<T> {
    pub fn new(x: T, y: T, z: T) -> Triple<T> {
        Triple(x, y, z)
    }
}

#[derive(Clone, Copy)]
pub struct Vec3(Triple<f64>);

pub struct ColorU8(Triple<u8>);
pub type Point3 = Vec3;
pub type ColorF64 = Vec3;

impl From<ColorU8> for ColorF64 {
    fn from(color: ColorU8) -> ColorF64 {
        let x = color.0 .0 as f64 / 255.9;
        let y = color.0 .1 as f64 / 255.9;
        let z = color.0 .2 as f64 / 255.9;

        ColorF64::new(x, y, z)
    }
}

impl From<ColorU8> for String {
    fn from(color: ColorU8) -> Self {
        format!("{} {} {}\n", color.0 .0, color.0 .1, color.0 .2)
    }
}

impl From<ColorF64> for ColorU8 {
    fn from(color: ColorF64) -> ColorU8 {
        let x = (color.0 .0 * 255.999) as u8;
        let y = (color.0 .1 * 255.999) as u8;
        let z = (color.0 .2 * 255.999) as u8;

        ColorU8(Triple(x, y, z))
    }
}

impl Vec3 {
    pub fn zero() -> Vec3 {
        Vec3(Triple(0., 0., 0.))
    }

    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3(Triple(x, y, z))
    }

    pub fn random() -> Vec3 {
        let mut rng = thread_rng();
        Vec3::new(rng.gen(), rng.gen(), rng.gen())
    }

    pub fn random_with(min: f64, max: f64) -> Vec3 {
        let mut rng = thread_rng();
        Vec3::new(
            rng.gen_range(min..max),
            rng.gen_range(min..max),
            rng.gen_range(min..max),
        )
    }

    pub fn random_point_on_unit_sphere() -> Vec3 {
        let mut rng = thread_rng();
        let (u, v) : (f64, f64) = rng.gen();

        let gamma = f64::acos(2. * u - 1.) - FRAC_PI_2;
        let phi = 2.0 * PI * v;

        let cos_gamma = f64::cos(gamma);


        Vec3::new(cos_gamma * f64::cos(phi), cos_gamma * f64::sin(phi), f64::sin(gamma))
    }

    pub fn x(&self) -> f64 {
        self.0 .0
    }

    pub fn y(&self) -> f64 {
        self.0 .1
    }

    pub fn z(&self) -> f64 {
        self.0 .2
    }

    pub fn length(self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    pub fn length_squared(self) -> f64 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }

    pub fn dot(self, rhs: &Vec3) -> f64 {
        self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z()
    }

    pub fn cross(self, rhs: Vec3) -> Vec3 {
        Vec3::new(
            self.y() * rhs.z() - self.z() * rhs.y(),
            self.y() * rhs.z() - self.z() * rhs.y(),
            self.y() * rhs.z() - self.z() * rhs.y(),
        )
    }

    pub fn unit_vector(self) -> Vec3 {
        self / self.length()
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.x(), -self.y(), -self.z())
    }
}

impl Deref for Vec3 {
    type Target = Triple<f64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x() * rhs, self.y() * rhs, self.z() * rhs)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z())
    }
}

impl Add<f64> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x() + rhs, self.y() + rhs, self.z() + rhs)
    }
}

impl Add<Vec3> for f64 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        rhs + self
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x() / rhs, self.y() / rhs, self.z() / rhs)
    }
}
