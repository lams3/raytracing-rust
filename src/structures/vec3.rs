use std::ops::{Neg, Add, AddAssign, Sub, SubAssign, Div, DivAssign, Mul, MulAssign, Index, IndexMut};
use std::default::Default;
use std::f64::consts::PI;

use rand::prelude::{thread_rng, Rng};

#[derive(PartialEq, Clone, Copy, Default, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64, 
    pub z: f64
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            x: x,
            y: y,
            z: z
        }
    }

    pub fn zero() -> Self {
        Self::default()
    }

    pub fn right() -> Self {
        Vec3::new(1.0, 0.0, 0.0)
    }

    pub fn left() -> Self {
        -Vec3::right()
    }

    pub fn up() -> Self {
        Vec3::new(0.0, 1.0, 0.0)
    }

    pub fn down() -> Self {
        -Vec3::up()
    }

    pub fn front() -> Self {
        Vec3::new(0.0, 0.0, 1.0)
    }

    pub fn back() -> Self {
        -Vec3::front()
    }

    pub fn random_in_unit_sphere() -> Self {
        let mut rng = thread_rng();
        let inclination = rng.gen_range(0.0..=PI);
        let azimuth = rng.gen_range(-PI..=PI);
        let r = 1.0;

        Self {
            x: r * inclination.sin() * azimuth.cos(),
            y: r * inclination.sin() * azimuth.sin(),
            z: r * inclination.cos()
        }
    }

    pub fn random_in_hemisphere(normal: &Self) -> Self {
        let in_unit_sphere = Self::random_in_unit_sphere();
        if Self::dot(&in_unit_sphere, normal) >= 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    pub fn random_inside_unit_disk() -> Vec3 {
        let mut rng = thread_rng();
        let theta = rng.gen_range(-PI..=PI);
        let radius = rng.gen_range(0.0..=1.0);
        radius * Vec3::new(theta.cos(), theta.sin(), 0.0)
    }

    pub fn reflect(v: &Self, n: &Self) -> Self{
        let v = *v;
        let n = *n;
        v - 2.0 * Self::dot(&v, &n) * n
    }

    pub fn refract(v: &Self, n: &Self, eta_in: f64, eta_out: f64) -> Self {
        let f = eta_in / eta_out;
        let v = *v;
        let n = *n;

        let cos_theta = f64::min(Vec3::dot(&(-v), &n), 1.0);
        let r_out_ortho = f * (v + cos_theta * n);
        let r_out_parallel = -f64::sqrt(f64::abs(1.0 - r_out_ortho.squared_length())) * n;
        
        r_out_ortho + r_out_parallel
    }

    pub fn dot(a: &Self, b: &Self) -> f64 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    pub fn cross(a: &Self, b: &Self) -> Self {
        Self {
            x: a.y * b.z - a.z * b.y,
            y: a.z * b.x - a.x * b.z,
            z: a.x * b.y - a.y * b.x
        }
    }

    pub fn lerp(a: &Self, b: &Self, t: f64) -> Self {
        let v1 = *a;
        let v2 = *b;
        (1.0 - t) * v1 + t * v2
    }

    pub fn squared_length(&self) -> f64 {
        Self::dot(self, self)
    }

    pub fn length(&self) -> f64 {
        self.squared_length().sqrt()
    }

    pub fn normalized(&self) -> Self {
        *self / self.length()
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        };
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, other: f64) -> Self::Output {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other
        }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        *self = Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        };
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Self::Output {
        other * self
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, other: f64) -> Self::Output {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        *self = Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        };
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, other: Vec3) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        }
    }
}

impl MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, other: Vec3) {
        *self = Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        };
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, i: usize) -> &Self::Output {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of range.")
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of range.")
        }
    }
}