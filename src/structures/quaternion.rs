use crate::structures::Vec3;

use std::ops::{Add, AddAssign, Sub, SubAssign, Div, DivAssign, Mul, MulAssign};

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Quaternion {
    pub v: Vec3,
    pub s: f64,
}

impl Quaternion {
    pub fn new(v: Vec3, s: f64) -> Self {
        Self {
            v: v,
            s: s
        }
    }

    pub fn from_axis_angle(axis: Vec3, angle: f64) -> Self{
        Quaternion::new(axis, angle).as_unit_norm()
    }

    pub fn as_unit_norm(&self) -> Self {
        let axis = self.v.normalized();
        let angle = self.s;
        let cos = f64::cos(angle / 2.0);
        let sin = f64::sin(angle / 2.0);

        Self {
            v: sin * axis,
            s: cos,
        }
    }

    pub fn squared_norm(&self) -> f64 {
        self.s.powi(2) + self.v.squared_length()
    }

    pub fn norm(&self) -> f64 {
        self.squared_norm().sqrt()
    }

    pub fn normalized(&self) -> Self {
        (*self) / self.norm()
    }

    pub fn conjugate(&self) -> Self {
        Self {
            v: - self.v,
            s: self.s
        }
    }

    pub fn inverse(&self) -> Self {
        self.conjugate() / self.squared_norm()
    }

    pub fn rotate_vector(&self, vec: Vec3) -> Vec3 {
        let q = self.normalized();
        let p = Quaternion::new(vec, 0.0);
        let rotated = q * p * q.inverse();
        rotated.v
    }
}

impl Default for Quaternion {

    fn default() -> Self {
        Self {
            v: Vec3::up(),
            s: 0.0
        }
    }
}

impl Add for Quaternion {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            v: self.v + other.v,
            s: self.s + other.s
        }
    }
}

impl AddAssign for Quaternion {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            v: self.v + other.v,
            s: self.s + other.s
        };
    }
}

impl Sub for Quaternion {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            v: self.v - other.v,
            s: self.s - other.s
        }
    }
}

impl SubAssign for Quaternion {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            v: self.v - other.v,
            s: self.s - other.s
        };
    }
}

impl Mul for Quaternion {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            v: self.s * other.v + other.s * self.v + Vec3::cross(&self.v, &other.v),
            s: self.s * other.s - Vec3::dot(&self.v, &other.v)
        }
    }
}

impl MulAssign for Quaternion {
    fn mul_assign(&mut self, other: Self) {
        *self = (*self) * other;
    }
}

impl Mul<Quaternion> for f64 {
    type Output = Quaternion;

    fn mul(self, quaternion: Quaternion) -> Self::Output {
        quaternion * self
    }
}

impl Mul<f64> for Quaternion {
    type Output = Self;

    fn mul(self, factor: f64) -> Self::Output {
        Self {
            v: self.v * factor,
            s: self.s * factor,
        }
    }
}

impl MulAssign<f64> for Quaternion {
    fn mul_assign(&mut self, factor: f64) {
        *self = Self {
            v: self.v * factor,
            s: self.s * factor
        };
    }
}

impl Div<f64> for Quaternion {
    type Output = Self;

    fn div(self, factor: f64) -> Self::Output {
        Self {
            v: self.v / factor,
            s: self.s / factor,
        }
    }
}

impl DivAssign<f64> for Quaternion {
    fn div_assign(&mut self, factor: f64) {
        *self = Self {
            v: self.v / factor,
            s: self.s / factor,
        };
    }
}