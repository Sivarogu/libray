extern crate image;

mod ray_tracing;

use ray_tracing::elements::{Scene, Color};
use ray_tracing::{Ray, Intersectable};

use image::{DynamicImage, GenericImage, Rgba, Pixel};

pub fn render(scene: &Scene) -> DynamicImage {
    let mut image = DynamicImage::new_rgb8(scene.width, scene.height);
    let black = Rgba::from_channels(0, 0, 0, 0);
    for x in 0..scene.width {
        for y in 0..scene.height {
            let ray = Ray::init_ray(x, y, scene);

            if scene.sphere.intersect(&ray) {
                image.put_pixel(x, y, to_rgba(&scene.sphere.color))
            } else {
                image.put_pixel(x, y, black);
            }
        }
    }
    image
}

fn to_rgba(color: &Color) -> Rgba<u8> {
    Rgba::from_channels(color.red, color.green, color.blue, 0)
}