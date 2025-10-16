mod vec3;

fn main() {
    let image_width = 256;
    let image_height = 256;

    println!("P3\n{} {}\n255\n", image_width, image_height);
    for j in 0..image_height {
        eprintln!("Scanlines remaining : {} ", (image_height - j));
        for i in 0..image_width {
            let r = (i as f64) / (image_width - 1) as f64;
            let g = (j as f64) / (image_height - 1) as f64;
            let b = 0.0;

            let ir = (255.999 * r) as i64;
            let ig = (255.999 * g) as i64;
            let ib = (255.999 * b) as i64;

            println!("{} {} {}", ir, ig, ib);
        }
    }
    eprintln!("Done.");
}
