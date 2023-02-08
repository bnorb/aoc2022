use std::{f64::consts::PI, ops::Mul};

#[derive(Clone)]
pub struct Quaternion {
    w: f64,
    i: f64,
    j: f64,
    k: f64,
}

impl Quaternion {
    fn new(w: f64, i: f64, j: f64, k: f64) -> Self {
        Self { w, i, j, k }
    }

    fn rotation_quaternions((i, j, k): (f64, f64, f64), degree: f64) -> (Self, Self) {
        let radian_half = 0.5 * PI * degree / 180.0;
        let sin = radian_half.sin();
        let cos = radian_half.cos();

        (
            Self::new(cos, sin * i, sin * j, sin * k),
            Self::new(cos, -sin * i, -sin * j, -sin * k),
        )
    }

    pub fn rotate_point(
        (x, y, z): (f64, f64, f64),
        axis: (f64, f64, f64),
        degree: f64,
        precision: i32,
    ) -> (f64, f64, f64) {
        if degree == 0.0 {
            return (x, y, z);
        }

        let (q, qi) = Self::rotation_quaternions(axis, degree);
        let rotated = (q * Self::new(0.0, x, y, z)) * qi;

        if precision == 0 {
            return (rotated.i.round(), rotated.j.round(), rotated.k.round());
        }

        let d = 10.0_f64.powi(precision);

        (
            (rotated.i * d).round() / d,
            (rotated.j * d).round() / d,
            (rotated.k * d).round() / d,
        )
    }
}

impl Mul for Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: Self) -> Self::Output {
        Quaternion::new(
            self.w * rhs.w - self.i * rhs.i - self.j * rhs.j - self.k * rhs.k,
            self.w * rhs.i + self.i * rhs.w + self.j * rhs.k - self.k * rhs.j,
            self.w * rhs.j - self.i * rhs.k + self.j * rhs.w + self.k * rhs.i,
            self.w * rhs.k + self.i * rhs.j - self.j * rhs.i + self.k * rhs.w,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_rotate_around_x() {
        assert_eq!(
            Quaternion::rotate_point((1.0, 2.0, 3.0), (1.0, 0.0, 0.0), 90.0, 0),
            (1.0, -3.0, 2.0)
        );
        assert_eq!(
            Quaternion::rotate_point((7.0, 2.0, -8.0), (1.0, 0.0, 0.0), -90.0, 0),
            (7.0, -8.0, -2.0)
        );
        assert_eq!(
            Quaternion::rotate_point((1.0, 2.0, 3.0), (-1.0, 0.0, 0.0), -90.0, 0),
            (1.0, -3.0, 2.0)
        );
        assert_eq!(
            Quaternion::rotate_point((7.0, 2.0, -8.0), (-1.0, 0.0, 0.0), 90.0, 0),
            (7.0, -8.0, -2.0)
        );
    }

    #[test]
    fn can_rotate_around_y() {
        assert_eq!(
            Quaternion::rotate_point((9.0, -342.0, 98.0), (0.0, 1.0, 0.0), 90.0, 0),
            (98.0, -342.0, -9.0)
        );
        assert_eq!(
            Quaternion::rotate_point((23.0, 55.0, 15.0), (0.0, 1.0, 0.0), -90.0, 0),
            (-15.0, 55.0, 23.0)
        );
        assert_eq!(
            Quaternion::rotate_point((9.0, -342.0, 98.0), (0.0, -1.0, 0.0), -90.0, 0),
            (98.0, -342.0, -9.0)
        );
        assert_eq!(
            Quaternion::rotate_point((23.0, 55.0, 15.0), (0.0, -1.0, 0.0), 90.0, 0),
            (-15.0, 55.0, 23.0)
        );
    }

    #[test]
    fn can_rotate_around_z() {
        assert_eq!(
            Quaternion::rotate_point((-45.0, 12.0, 444.0), (0.0, 0.0, 1.0), 90.0, 0),
            (-12.0, -45.0, 444.0)
        );
        assert_eq!(
            Quaternion::rotate_point((88.0, 234.0, 245.0), (0.0, 0.0, 1.0), -90.0, 0),
            (234.0, -88.0, 245.0)
        );
        assert_eq!(
            Quaternion::rotate_point((-45.0, 12.0, 444.0), (0.0, 0.0, -1.0), -90.0, 0),
            (-12.0, -45.0, 444.0)
        );
        assert_eq!(
            Quaternion::rotate_point((88.0, 234.0, 245.0), (0.0, 0.0, -1.0), 90.0, 0),
            (234.0, -88.0, 245.0)
        );
    }
}
