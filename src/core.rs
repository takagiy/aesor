use crate::util::Vec3;
use image::{Pixel, Rgba, RgbaImage};

pub trait Reflect {
    fn normal_vec(&self, p: &Vec3) -> Option<(Vec3, f64)>;
    fn reflect(&self, p: &Vec3, incident: &Vec3, sight: &Vec3) -> Option<(f64, f64, f64)> {
        self.normal_vec(p).map(|(n, alpha)| {
            let reflection = (incident - &(2. * incident.dot(&n) * &n)).normal();
            (-incident.dot(&n), reflection.dot(&-sight).max(0.), alpha)
        })
    }
}

impl<T: Reflect> Reflect for Box<T> {
    fn normal_vec(&self, p: &Vec3) -> Option<(Vec3, f64)> {
        self.as_ref().normal_vec(p)
    }

    fn reflect(&self, p: &Vec3, incident: &Vec3, sight: &Vec3) -> Option<(f64, f64, f64)> {
        self.as_ref().reflect(p, incident, sight)
    }
}

pub trait Draw {
    fn draw(&self, setting: &Setting, material: &Material, img: &mut RgbaImage);
}

impl<T: Reflect> Draw for T {
    fn draw(&self, setting: &Setting, material: &Material, img: &mut RgbaImage) {
        let (w, h) = img.dimensions();
        for (x, y, px) in img.enumerate_pixels_mut() {
            if let Some((df, sp, alpha)) = self.reflect(
                &Vec3::new(x as f64, y as f64, 0.),
                &setting.incident,
                &Vec3::new(
                    x as f64 - w as f64 / 2.,
                    y as f64 - h as f64 / 2.,
                    -(setting.distance as f64),
                )
                .normal(),
            ) {
                let df = 0.3 * df + setting.ambient_brightness;
                let r = (material.color[0] as f64 * df
                    + material.reflection_brightness * 255. * sp.powi(material.shininess))
                .min(255.) as u8;
                let g = (material.color[1] as f64 * df
                    + material.reflection_brightness * 255. * sp.powi(material.shininess))
                .min(255.) as u8;
                let b = (material.color[2] as f64 * df
                    + material.reflection_brightness * 255. * sp.powi(material.shininess))
                .min(255.) as u8;
                let a = (material.color[3] as f64 * alpha).min(255.) as u8;
                px.blend(&Rgba([r, g, b, a]));
            }
        }
    }
}

impl<E: Draw> Draw for Vec<E> {
    fn draw(&self, setting: &Setting, material: &Material, img: &mut RgbaImage) {
        for e in self {
            e.draw(setting, material, img);
        }
    }
}

#[derive(Clone)]
pub struct Material {
    pub color: Rgba<u8>,
    pub shininess: i32,
    pub reflection_brightness: f64,
}

pub struct Setting {
    pub distance: u32,
    pub incident: Vec3,
    pub ambient_brightness: f64,
}

pub struct Object<T: Draw> {
    pub shape: T,
    pub material: Material,
}

impl<T: Draw> Object<T> {
    pub fn draw(&self, setting: &Setting, img: &mut RgbaImage) {
        self.shape.draw(setting, &self.material, img);
    }
}

pub trait IntoObject {
    type Shape: Draw;
    fn with(self, material: Material) -> Object<Self::Shape>;
}

impl<T: Draw> IntoObject for T {
    type Shape = T;
    fn with(self, material: Material) -> Object<Self::Shape> {
        Object {
            shape: self,
            material,
        }
    }
}
