#![allow(unused_imports, unused_variables, dead_code)]
use std::f64::INFINITY;
use std::sync::Arc;

use crate::camera::Camera;
use crate::color::*;
use crate::hittable::*;
use crate::hittable_list::*;
use crate::interval::Interval;
use crate::material::Dielectic;
use crate::material::Lambertian;
use crate::material::Metal;
use crate::ray::*;
use crate::rtweekend::*;
use crate::sphere::*;
use crate::vec3::*;

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;

fn main() {
    let mut world: HittableList = HittableList::new();

    let r = core::f64::consts::FRAC_PI_4; // 90 deg

    // let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    // let material_center = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    // let material_left = Dielectic::new(1.5);
    // let material_bubble = Dielectic::new(1.0 / 1.5);
    // let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 1.0);

    let material_left = Lambertian::new(Color::new(0.0, 0.0, 1.0));
    let material_right = Lambertian::new(Color::new(1.0, 0.0, 0.0));
    // world.add(Arc::new(Sphere::new(
    //     Point3::new(0.0, -100.5, -1.0),
    //     100.0,
    //     Arc::new(material_ground),
    // )));
    //
    // world.add(Arc::new(Sphere::new(
    //     Point3::new(0.0, 0.0, -1.2),
    //     0.5,
    //     Arc::new(material_center),
    // )));
    //
    // world.add(Arc::new(Sphere::new(
    //     Point3::new(-1.0, 0.0, -1.0),
    //     0.5,
    //     Arc::new(material_left),
    // )));
    // world.add(Arc::new(Sphere::new(
    //     Point3::new(-1.0, 0.0, -1.0),
    //     0.4,
    //     Arc::new(material_bubble),
    // )));
    //
    // world.add(Arc::new(Sphere::new(
    //     Point3::new(1.0, 0.0, -1.0),
    //     0.5,
    //     Arc::new(material_right),
    // )));

    world.add(Arc::new(Sphere::new(
        Point3::new(-r, 0.0, -1.0),
        r,
        Arc::new(material_left),
    )));

    world.add(Arc::new(Sphere::new(
        Point3::new(r, 0.0, -1.0),
        r,
        Arc::new(material_right),
    )));
    let mut camera = Camera::new();

    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;
    camera.fov = 90.0;
    camera.render(&world);
}
