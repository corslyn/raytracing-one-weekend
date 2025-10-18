use std::f64::INFINITY;
use std::sync::Arc;

use crate::camera::Camera;
use crate::color::*;
use crate::hittable::*;
use crate::hittable_list::*;
use crate::interval::Interval;
use crate::ray::*;
use crate::rtweekend::*;
use crate::sphere::*;
use crate::vec3::*;

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;

fn main() {
    let mut world: HittableList = HittableList::new();
    world.add(Arc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let mut camera = Camera::new();

    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;

    camera.render(&world);
}
