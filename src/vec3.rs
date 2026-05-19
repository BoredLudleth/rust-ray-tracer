use std::fmt;
use std::ops;
use std::f32;

use crate::rtweekend::random_double_range;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    e: [f32; 3],
}

impl Vec3 {
    pub fn new() -> Self {
        Self { e: [0.0, 0.0, 0.0] }
    }
    
    pub fn from(e0: f32, e1: f32, e2: f32) -> Self {
        Self { e: [e0, e1, e2] }
    }
    
    pub fn x(&self) -> f32 { self.e[0] }
    pub fn y(&self) -> f32 { self.e[1] }
    pub fn z(&self) -> f32 { self.e[2] }
    
    pub fn r(&self) -> f32 { self.e[0] }
    pub fn g(&self) -> f32 { self.e[1] }
    pub fn b(&self) -> f32 { self.e[2] }
    
    pub fn positive(&self) -> Self { *self }

    pub fn length(&self) -> f32 {
        (self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]).sqrt()
    }
    
    pub fn squared_length(&self) -> f32 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }
    
    pub fn make_unit_vector(&mut self) {
        let k = 1.0 / (self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]).sqrt();
        self.e[0] *= k;
        self.e[1] *= k;
        self.e[2] *= k;
    }

    pub fn random() -> Self {
        Self::from(
            crate::rtweekend::random_double(),
            crate::rtweekend::random_double(),
            crate::rtweekend::random_double(),
        )
    }
    
    pub fn random_range(min: f32, max: f32) -> Self {
        Self::from(
            random_double_range(min, max),
            random_double_range(min, max),
            random_double_range(min, max),
        )
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.e[0].abs() < s && self.e[1].abs() < s && self.e[2].abs() < s
    }
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * dot(&v, &n) * n
}

pub fn random_unit_vector() -> Vec3 {
    loop {
        let p = Vec3::random_range(-1.0, 1.0);
        let lensq = p.squared_length();
        if lensq > 1e-160 && lensq <= 1.0 {
            return p / lensq.sqrt();
        }
    }
}

pub fn random_on_hemisphere(normal: Vec3) -> Vec3 {
    let on_unit_sphere = random_unit_vector();
    if dot(&on_unit_sphere, &normal) > 0.0 {
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f32;
    
    fn index(&self, i: usize) -> &f32 {
        &self.e[i]
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut f32 {
        &mut self.e[i]
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;
    
    fn neg(self) -> Self {
        Self::from(-self.e[0], -self.e[1], -self.e[2])
    }
}

impl ops::Add for Vec3 {
    type Output = Self;
    
    fn add(self, other: Self) -> Self {
        Self::from(
            self.e[0] + other.e[0],
            self.e[1] + other.e[1],
            self.e[2] + other.e[2],
        )
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;
    
    fn sub(self, other: Self) -> Self {
        Self::from(
            self.e[0] - other.e[0],
            self.e[1] - other.e[1],
            self.e[2] - other.e[2],
        )
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;
    
    fn mul(self, other: Self) -> Self {
        Self::from(
            self.e[0] * other.e[0],
            self.e[1] * other.e[1],
            self.e[2] * other.e[2],
        )
    }
}

impl ops::Div for Vec3 {
    type Output = Self;
    
    fn div(self, other: Self) -> Self {
        Self::from(
            self.e[0] / other.e[0],
            self.e[1] / other.e[1],
            self.e[2] / other.e[2],
        )
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;
    
    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::from(self * v.e[0], self * v.e[1], self * v.e[2])
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Self;
    
    fn mul(self, t: f32) -> Self {
        Self::from(self.e[0] * t, self.e[1] * t, self.e[2] * t)
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Self;
    
    fn div(self, t: f32) -> Self {
        Self::from(self.e[0] / t, self.e[1] / t, self.e[2] / t)
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.e[0] += other.e[0];
        self.e[1] += other.e[1];
        self.e[2] += other.e[2];
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        self.e[0] -= other.e[0];
        self.e[1] -= other.e[1];
        self.e[2] -= other.e[2];
    }
}

impl ops::MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Self) {
        self.e[0] *= other.e[0];
        self.e[1] *= other.e[1];
        self.e[2] *= other.e[2];
    }
}

impl ops::DivAssign for Vec3 {
    fn div_assign(&mut self, other: Self) {
        self.e[0] /= other.e[0];
        self.e[1] /= other.e[1];
        self.e[2] /= other.e[2];
    }
}

impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, t: f32) {
        self.e[0] *= t;
        self.e[1] *= t;
        self.e[2] *= t;
    }
}

impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, t: f32) {
        let k = 1.0 / t;
        self.e[0] *= k;
        self.e[1] *= k;
        self.e[2] *= k;
    }
}

pub fn dot(v1: &Vec3, v2: &Vec3) -> f32 {
    v1.e[0] * v2.e[0] + v1.e[1] * v2.e[1] + v1.e[2] * v2.e[2]
}

pub fn cross(v1: &Vec3, v2: &Vec3) -> Vec3 {
    Vec3::from(
        v1.e[1] * v2.e[2] - v1.e[2] * v2.e[1],
        v1.e[2] * v2.e[0] - v1.e[0] * v2.e[2],
        v1.e[0] * v2.e[1] - v1.e[1] * v2.e[0],
    )
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}

pub type Point3 = Vec3;
