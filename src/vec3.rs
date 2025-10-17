use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg},
};

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}
pub type Color = Vec3;
pub type Point3 = Vec3;

impl Vec3 {
    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
    pub fn z(&self) -> f64 {
        self.z
    }

    fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Vec3 {
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, v: Self) -> Self::Output {
        Vec3::new(self.x + v.x, self.y + v.y, self.z + v.z)
    }
}

impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, v: Self) -> Self::Output {
        Vec3::new(self.x * v.x, self.y * v.y, self.z * v.z)
    }
}
impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, t: f64) -> Self::Output {
        Vec3::new(self.x * t, self.y * t, self.z * t)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Self::Output {
        Vec3::new(v.x * self, v.y * self, v.z * self)
    }
}
impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x as i64, self.y as i64, self.z as i64)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, v: Self) {
        self.x += v.x;
        self.y += v.y;
        self.z += v.z;
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t: f64) {
        self.x *= t;
        self.y *= t;
        self.z *= t;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t: f64) {
        *self *= 1.0 / t;
    }
}

pub fn dot(u: Vec3, v: Vec3) -> f64 {
    u.x * v.x + u.y * v.y + u.z * v.z
}

pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3::new(
        u.y * v.z - u.z * v.y,
        u.z * v.x - u.x * v.z,
        u.x * v.y - u.y * v.x,
    )
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    let len = v.length();
    v / len
}
