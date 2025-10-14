use cgmath::Vector3;

pub struct Color {
    pub color: Vector3<f32>,
}

pub fn write_color(color: Vector3<f32>) {
    let r = color.x;
    let g = color.y;
    let b = color.z;
    let rbyte = (255.999 * r) as i32;
    let gbyte = (255.999 * g) as i32;
    let bbyte = (255.999 * b) as i32;
    print!("{} {} {}\n", rbyte, gbyte, bbyte);
}
