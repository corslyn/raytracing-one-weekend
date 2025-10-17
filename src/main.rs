use crate::color::*;
use crate::ray::*;
use crate::vec3::*;

mod color;
mod ray;
mod vec3;

fn ray_color(r: Ray) -> Color {
    let unit_direction = unit_vector(r.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let image_height = if image_height < 1 { 1 } else { image_height };

    // Camera

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_width, 0.0);

    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;

    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    println!("P3\n{} {}\n255\n", image_width, image_height);
    for j in 0..image_height {
        eprintln!("Scanlines remaining : {} ", (image_height - j));
        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            let pixel_color: Color = ray_color(r);
            write_color(pixel_color);
        }
    }
    eprintln!("Done.");
}
