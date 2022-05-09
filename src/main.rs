use std::ops::{Add, Div, Mul, Sub};

use image::{ImageBuffer, ImageFormat, Pixel, Rgba, RgbaImage};

#[derive(Clone)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

fn point(x: f64, y: f64) -> Vec3 {
    Vec3 { x, y, z: 0. }
}

impl Vec3 {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    fn dot(&self, rhs: &Vec3) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    fn norm(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    fn normal(&self) -> Self {
        self / self.norm()
    }

    fn acos_xy(&self) -> f64 {
        (self.x / self.norm()).acos()
    }

    fn asin_xy(&self) -> f64 {
        (self.y / self.norm()).asin()
    }

    fn rot_xy(&self, rot: &Vec3) -> Self {
        let nrot = rot.normal();
        Vec3 {
            x: self.x * nrot.x - self.y * nrot.y,
            y: self.x * nrot.y + self.y * nrot.x,
            z: self.z,
        }
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

trait Reflect {
    fn reflect(&self, p: &Vec3, incident: &Vec3) -> Option<(f64, f64)>;

    fn draw(&self, incident: &Vec3, img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) {
        for (x, y, px) in img.enumerate_pixels_mut() {
            if let Some((br, alpha)) = self.reflect(&Vec3::new(x as f64, y as f64, 0.), incident) {
                let br = (50. * br) as u8 + 200;
                px.blend(&Rgba([br, br, br, (255. * alpha) as u8]));
            }
        }
    }
}

struct Corn {
    center: Vec3,
    radius: f64,
    height: f64,
}

impl Reflect for Corn {
    fn reflect(&self, p: &Vec3, incident: &Vec3) -> Option<(f64, f64)> {
        let p = p - &self.center;
        if p.norm() <= self.radius {
            let n = Vec3::new(self.height, 0., self.radius).rot_xy(&p).normal();
            Some((-incident.dot(&n), (self.radius - p.norm()).min(1.)))
        } else {
            None
        }
    }
}

struct Concave {
    center: Vec3,
    radius: f64,
    depth: f64,
}

impl Reflect for Concave {
    fn reflect(&self, p: &Vec3, incident: &Vec3) -> Option<(f64, f64)> {
        let p = p - &self.center;
        if p.norm() <= self.radius {
            let y0 = (self.radius.powi(2) - self.depth.powi(2)) / (2. * self.depth.abs());
            let r = self.depth.abs() + y0;
            let theta = (p.norm() / r).asin();
            let p_sign = -1. * self.depth / self.depth.abs();
            let n = Vec3::new(p_sign * p.x, p_sign * p.y, r * theta.cos()).normal();
            Some((-incident.dot(&n), (self.radius - p.norm()).min(1.)))
        } else {
            None
        }
    }
}

struct RoundBox {
    top_left: Vec3,
    bottom_right: Vec3,
    border_radius: f64,
    depth: f64,
}

impl Reflect for RoundBox {
    fn reflect(&self, p: &Vec3, incident: &Vec3) -> Option<(f64, f64)> {
        let (rim_x, rim_y) = if p.x <= self.top_left.x {
            if p.y <= self.top_left.y {
                (self.top_left.x, self.top_left.y)
            } else if p.y <= self.bottom_right.y {
                (self.top_left.x, p.y)
            } else {
                (self.top_left.x, self.bottom_right.y)
            }
        } else if p.x <= self.bottom_right.x {
            if p.y <= self.top_left.y {
                (p.x, self.top_left.y)
            } else if p.y <= self.bottom_right.y {
                (p.x, p.y)
            } else {
                (p.x, self.bottom_right.y)
            }
        } else {
            if p.y <= self.top_left.y {
                (self.bottom_right.x, self.top_left.y)
            } else if p.y <= self.bottom_right.y {
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
        rim.reflect(p, incident)
    }
}

fn main() {
    let rim = Corn {
        center: point(150., 150.),
        radius: 150.,
        height: 450.,
    };
    let top = Concave {
        center: point(150., 150.),
        radius: 130.,
        depth: 30.,
    };
    let tip = Concave {
        center: point(215., 85.),
        radius: 15.,
        depth: 15.,
    };
    let incident = Vec3::new(0., 1., -1.).normal();
    let mut img = RgbaImage::new(300, 300);
    rim.draw(&incident, &mut img);
    top.draw(&incident, &mut img);
    tip.draw(&incident, &mut img);
    img.save_with_format("out.png", ImageFormat::Png).unwrap();
    let rim = RoundBox {
        top_left: point(50., 100.),
        bottom_right: point(250., 100.),
        border_radius: 40.,
        depth: -40.,
    };
    let trace = RoundBox {
        top_left: point(50., 100.),
        bottom_right: point(250., 100.),
        border_radius: 25.,
        depth: 25.,
    };
    let slider = RoundBox {
        top_left: point(50., 100.),
        bottom_right: point(150., 100.),
        border_radius: 20.,
        depth: -20.,
    };
    let top = RoundBox {
        top_left: point(50., 100.),
        bottom_right: point(150., 100.),
        border_radius: 15.,
        depth: 5.
    };
    let tips: Vec<_> = (0..=0)
        .into_iter()
        .map(|i| RoundBox {
            top_left: point(100. + 12. * i as f64, 90.),
            bottom_right: point(100. + 12. * i as f64, 110.),
            border_radius: 4.,
            depth: -4.,
        })
        .collect();
    let mut img = RgbaImage::new(300, 200);
    rim.draw(&incident, &mut img);
    trace.draw(&incident, &mut img);
    slider.draw(&incident, &mut img);
    top.draw(&incident, &mut img);
    for t in &tips {
        t.draw(&incident, &mut img);
    }
    img.save_with_format("out2.png", ImageFormat::Png).unwrap();
}
