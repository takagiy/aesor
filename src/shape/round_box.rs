use crate::shape::Concave;
use crate::util::{point, Vec3};
use crate::Reflect;

pub struct RoundBox {
    pub top_left: Vec3,
    pub bottom_right: Vec3,
    pub border_radius: f64,
    pub depth: f64,
}

impl Reflect for RoundBox {
    fn normal_vec(&self, p: &Vec3) -> Option<(Vec3, f64)> {
        let (rim_x, rim_y) = if p.x <= self.top_left.x {
            if p.y <= self.top_left.y {
                (self.top_left.x, self.top_left.y)
            } else if p.y < self.bottom_right.y {
                (self.top_left.x, p.y)
            } else {
                (self.top_left.x, self.bottom_right.y)
            }
        } else if p.x < self.bottom_right.x {
            if p.y <= self.top_left.y {
                (p.x, self.top_left.y)
            } else if p.y < self.bottom_right.y {
                (p.x, p.y)
            } else {
                (p.x, self.bottom_right.y)
            }
        } else {
            if p.y <= self.top_left.y {
                (self.bottom_right.x, self.top_left.y)
            } else if p.y < self.bottom_right.y {
                (self.bottom_right.x, p.y)
            } else {
                (self.bottom_right.x, self.bottom_right.y)
            }
        };
        let rim = Concave {
            center: point(rim_x, rim_y),
            radius: self.border_radius,
            depth: self.depth,
        };
        rim.normal_vec(p)
    }
}
