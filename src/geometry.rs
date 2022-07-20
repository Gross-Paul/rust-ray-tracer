use std::ops;

/// A 3D Point using f64
struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}

impl Point3D {
    /// Constructor for Point3D
    fn new(x: f64, y: f64, z: f64) -> Point3D {
        Point3D { x, y, z }
    }

    /// Creates a Point at 0, 0, 0
    fn zero() -> Point3D {
        Point3D::new(0., 0., 0.)
    }

    /// The amplitude of the vector or the distance between zero and this point
    pub fn amplitude(&self) -> f64 {
        f64::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }

}

impl ops::Add<Point3D> for Point3D {
    type Output = Point3D;

    /// Bitwise sum
    fn add(self, _rhs: Point3D) -> Point3D {
        Point3D { x: (self.x + _rhs.x), y: (self.y + _rhs.y), z: (self.z + _rhs.z) }
    }
}

#[cfg(test)]
mod tests {
    /// Assert float equal
    macro_rules! assert_float_equal {
        ($a:expr, $b:expr) => {{
            let eps: f64 = 10e-5;
            assert!(f64::abs($a - $b) < eps);
        }};
    }
    mod geometry {
        use crate::geometry::Point3D;

        /// Is the new constructor working as expected
        #[test]
        fn new_is_new() {
            let x = 5.;
            let y = -50.;
            let z = 100.;
            let point = Point3D::new(x, y, z);

            assert_float_equal!(x, point.x);
            assert_float_equal!(y, point.y);
            assert_float_equal!(z, point.z);
        }

        /// Is the zero constructor working as expected
        #[test]
        fn zero_is_indeed_zero() {
            let point = Point3D::zero();

            assert_float_equal!(0., point.x);
            assert_float_equal!(0., point.y);
            assert_float_equal!(0., point.z);
        }

        #[test]
        fn add_is_add() {
            let p1 = Point3D::new(1., -5., 10.);
            let p2 = Point3D::new(-0.5, 1.7, 20000.);

            let p1_add_p2 = p1 + p2;

            assert_float_equal!(0.5, p1_add_p2.x);
            assert_float_equal!(-3.3, p1_add_p2.y);
            assert_float_equal!(20010., p1_add_p2.z);
        }

        #[test]
        fn test_amplitude() {
            let p1 = Point3D::new(5., 0., 0.);
            assert_float_equal!(5., p1.amplitude());
            
            let p1 = Point3D::new(-3., 4., 9.);
            assert_float_equal!(f64::sqrt(106.), p1.amplitude());
        }
    }
}
