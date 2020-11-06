use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub};

use crate::m::{rand_f, rand_f_mm};

//Axis
pub enum Axis {
    X,
    Y,
    Z,
}


//Colors
pub enum Colors {
    R,
    G,
    B,
}

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    e: [f64; 3]
}

impl Index<Axis> for Vec3 {
    type Output = f64;

    fn index(&self, index: Axis) -> &Self::Output {
        match index {
            Axis::X => &self.e[0],
            Axis::Y => &self.e[1],
            Axis::Z => &self.e[2],
        }
    }
}

impl Index<Colors> for Vec3 {
    type Output = f64;

    fn index(&self, index: Colors) -> &Self::Output {
        match index {
            Colors::R => &self.e[0],
            Colors::G => &self.e[1],
            Colors::B => &self.e[2],
        }
    }
}

impl IndexMut<Colors> for Vec3 {
    fn index_mut(&mut self, index: Colors) -> &mut Self::Output {
        match index {
            Colors::R => &mut self.e[0],
            Colors::G => &mut self.e[1],
            Colors::B => &mut self.e[2],
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self.e[0] + rhs.e[0], self.e[1] + rhs.e[1], self.e[2] + rhs.e[2])
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(self.e[0] - rhs.e[0], self.e[1] - rhs.e[1], self.e[2] - rhs.e[2])
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3::new(self.e[0] * rhs.e[0], self.e[1] * rhs.e[1], self.e[2] * rhs.e[2])
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}


impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
    }
}


impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        (1.0 / rhs) * self
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}


impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.e[0], -self.e[1], -self.e[2])
    }
}

impl Vec3 {
    pub fn zero() -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }

    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }

    pub fn random() -> Vec3 {
        Vec3::new(rand_f(), rand_f(), rand_f())
    }

    pub fn random_mm(min: f64, max: f64) -> Vec3 {
        Vec3::new(rand_f_mm(min, max), rand_f_mm(min, max), rand_f_mm(min, max))
    }

    pub fn rand_in_unit_sphere() -> Vec3 {
        let mut vec = Option::None;
        while vec.is_none() {
            let p = Vec3::random_mm(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                vec = Some(p)
            }
        }
        vec.unwrap()
    }

    pub fn random_in_unit_disk() -> Vec3 {
        let mut vec = Option::None;
        while vec.is_none() {
            let p = Vec3::new(rand_f_mm(-1.0, 1.0), rand_f_mm(-1.0, 1.0), 0.0);
            if p.length_squared() < 1.0 {
                vec = Some(p)
            }
        }
        vec.unwrap()
    }

    pub fn random_unit_vector() -> Vec3 {
        Vec3::rand_in_unit_sphere().unit_vec()
    }


    pub fn length_squared(self) -> f64 {
        self.e[0] * self.e[0] +
            self.e[1] * self.e[1] +
            self.e[2] * self.e[2]
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(self, v: Vec3) -> f64 {
        self.e[0] * v.e[0] +
            self.e[1] * v.e[1] +
            self.e[2] * v.e[2]
    }

    pub fn cross(self, v: Vec3) -> Vec3 {
        Vec3::new(self.e[1] * v.e[2] - self.e[2] * v.e[1],
                  self.e[2] * v.e[0] - self.e[0] * v.e[2],
                  self.e[0] * v.e[1] - self.e[1] * v.e[0])
    }

    pub fn unit_vec(self) -> Vec3 {
        self / self.length()
    }

    pub fn near_zero(self) -> bool {
        let s = 1e-8;
        self.e[0].abs() < s && self.e[1].abs() < s && self.e[2].abs() < s
    }

    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        v - 2.0 * v.dot(n) * n
    }

    pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = ((-uv).dot(n)).min(1.0);
        let r_out_perp = etai_over_etat * (uv + cos_theta * n);
        let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * n;
        r_out_perp + r_out_parallel
    }
}


pub type Point3 = Vec3;
pub type Color3 = Vec3;