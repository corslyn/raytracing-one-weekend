use crate::vec3::Color;

pub fn write_color(pixel_color: Color) {
    let ir = (255.999 * pixel_color.x()) as i64;
    let ig = (255.999 * pixel_color.y()) as i64;
    let ib = (255.999 * pixel_color.z()) as i64;

    println!("{} {} {}", ir, ig, ib);
}
