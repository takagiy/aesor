use crate::util::Vec3;
use crate::Reflect;

pub struct Concave {
    pub center: Vec3,
    pub radius: f64,
    pub depth: f64,
}

impl Reflect for Concave {
    fn normal_vec(&self, p: &Vec3) -> Option<(Vec3, f64)> {
        let p = p - &self.center;
        if p.norm() <= self.radius {
            let y0 = (self.radius.powi(2) - self.depth.powi(2)) / (2. * self.depth.abs());
            let r = self.depth.abs() + y0;
            let theta = (p.norm() / r).asin();
            let p_sign = -1. * self.depth / self.depth.abs();
            let n = Vec3::new(p_sign * p.x, p_sign * p.y, r * theta.cos()).normal();
            Some((n, (self.radius - p.norm()).min(1.)))
        } else {
            None
        }
    }
}
