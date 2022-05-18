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
    let black = Material {
        color: Rgba([0, 0, 0, 255]),
        shininess: 7,
        reflection_brightness: 1.,
    };
    let blue = Material {
        color: Rgba([179, 220, 214, 255]),
        shininess: 4,
        reflection_brightness: 0.2,
    };

    let mut img = RgbaImage::new(300, 200);

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
    vec![rim, trace].with(blue).draw(&setting, &mut img);

    let slider_rim = RoundBox {
        top_left: point(50., 100.),
        bottom_right: point(150., 100.),
        border_radius: 20.,
        depth: -20.,
    };
    let slider_top = RoundBox {
        top_left: point(50., 100.),
        bottom_right: point(150., 100.),
        border_radius: 15.,
        depth: 5.,
    };
    let tips: Vec<_> = (-1..=1)
        .into_iter()
        .map(|i| RoundBox {
            top_left: point(100. + 12. * i as f64, 90.),
            bottom_right: point(100. + 12. * i as f64, 110.),
            border_radius: 4.,
            depth: -4.,
        })
        .collect();

    vec![slider_rim, slider_top]
        .with(black.clone())
        .draw(&setting, &mut img);
    tips.with(black).draw(&setting, &mut img);

    img.save_with_format("slider.png", ImageFormat::Png)
        .unwrap();
}
