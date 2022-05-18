use aesor::shape::*;
use aesor::util::{point, Vec3};
use aesor::*;
use image::{ImageFormat, Rgba, RgbaImage};

fn main() {
    let setting = Setting {
        incident: Vec3::new(0.2, 1., -0.2),
        ambient_brightness: 0.8,
        distance: 2000,
    };
    let white = Material {
        color: Rgba([255, 255, 255, 255]),
        shininess: 7,
        reflection_brightness: 1.,
    };
    let black = Material {
        color: Rgba([0, 0, 0, 255]),
        shininess: 7,
        reflection_brightness: 1.,
    };

    let mut img = RgbaImage::new(300, 300);

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
    let tip_rim = Corn {
        center: point(215., 85.),
        radius: 20.,
        height: -30.,
    };
    rim.with(white.clone()).draw(&setting, &mut img);
    top.with(white.clone()).draw(&setting, &mut img);
    tip_rim.with(white).draw(&setting, &mut img);

    let tip = Concave {
        center: point(215., 85.),
        radius: 15.,
        depth: 2.5,
    };
    tip.with(black).draw(&setting, &mut img);

    img.save_with_format("dial.png", ImageFormat::Png).unwrap();
}
