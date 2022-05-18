use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub fn point(x: f64, y: f64) -> Vec3 {
    Vec3 { x, y, z: 0. }
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    pub fn dot(&self, rhs: &Vec3) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn norm(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn normal(&self) -> Self {
        self / self.norm()
    }

    pub fn acos_xy(&self) -> f64 {
        (self.x / self.norm()).acos()
    }

    pub fn asin_xy(&self) -> f64 {
        (self.y / self.norm()).asin()
    }

    pub fn rot_xy(&self, rot: &Vec3) -> Self {
        let nrot = rot.normal();
        Vec3 {
            x: self.x * nrot.x - self.y * nrot.y,
            y: self.x * nrot.y + self.y * nrot.x,
            z: self.z,
        }
    }
}

impl Neg for &Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        -1. * self
    }
}

impl Add for &Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for &Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<&Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Div<f64> for &Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}
