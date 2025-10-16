use crate::color::*;

mod color;
mod vec3;

fn main() {
    let image_width = 256;
    let image_height = 256;

    println!("P3\n{} {}\n255\n", image_width, image_height);
    for j in 0..image_height {
        eprintln!("Scanlines remaining : {} ", (image_height - j));
        for i in 0..image_width {
            let pixel_color = vec3::Vec3::new(
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                0.0,
            );
            write_color(pixel_color);
        }
    }
    eprintln!("Done.");
}
