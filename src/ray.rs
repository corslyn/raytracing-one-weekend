use cgmath::{Point3, Vector3};

pub struct Ray {
    origin: Point3<f32>,
    direction: Vector3<f32>,
}

pub fn origin(ray: &Ray) -> Point3<f32> {
    ray.origin
}

pub fn direction(ray: &Ray) -> Vector3<f32> {
    ray.direction
}

pub fn at(ray: &Ray, t: f32) -> Point3<f32> {
    ray.origin + t * ray.direction
}

pub fn new(origin: Point3<f32>, direction: Vector3<f32>) -> Ray {
    Ray { origin, direction }
}
