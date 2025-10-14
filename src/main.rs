use cgmath::InnerSpace;
use log::info;

use crate::color::Color;

mod color;
mod ray;

fn ray_color(ray: &ray::Ray) -> Color {
    let unit_direction = ray::direction(ray).normalize();
    let a = 0.5 * (unit_direction.y + 1.0);
    let c = (1.0 - a)
        * cgmath::Vector3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
        + a * cgmath::Vector3 {
            x: 0.5,
            y: 0.7,
            z: 1.0,
        };
    Color { color: c }
}

fn main() {
    env_logger::init();

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let mut image_height: i32 = (image_width as f32 / aspect_ratio) as i32;
    image_height = ((image_height as f32 + 0.5).floor()) as i32;

    // camera

    let focal = 1.0;
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let camera_origin = cgmath::Point3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    let viewport_u = cgmath::Vector3 {
        x: viewport_width,
        y: 0.0,
        z: 0.0,
    };
    let viewport_v = cgmath::Vector3 {
        x: 0.0,
        y: -viewport_height,
        z: 0.0,
    };

    let pixel_delta_u = viewport_u / image_width as f32;
    let pixel_delta_v = viewport_v / image_height as f32;

    let viewport_upper_left = camera_origin
        - cgmath::Vector3 {
            x: 0.0,
            y: 0.0,
            z: focal,
        }
        - viewport_u / 2.0
        - viewport_v / 2.0;

    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);
    // render

    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in 0..image_height {
        info!("Scanlines remaining: {} ", (image_height - j));
        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (i as f32) * pixel_delta_u + (j as f32) * pixel_delta_v;
            let ray_direction = pixel_center - camera_origin;
            let r = ray::new(camera_origin, ray_direction);
            let pixel_color = ray_color(&r);
            color::write_color(pixel_color.color);
        }
    }
    info!("Done.")
}
