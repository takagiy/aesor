use crate::util::Vec3;
use crate::Reflect;

pub struct Corn {
    pub center: Vec3,
    pub radius: f64,
    pub height: f64,
}

impl Reflect for Corn {
    fn normal_vec(&self, p: &Vec3) -> Option<(Vec3, f64)> {
        let p = p - &self.center;
        if p.norm() <= self.radius {
            let n = Vec3::new(self.height, 0., self.radius).rot_xy(&p).normal();
            Some((n, (self.radius - p.norm()).min(1.)))
        } else {
            None
        }
    }
}
