use core::f64;
use std::f64::consts::PI;

use rand::{Rng, rng};

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_double() -> f64 {
    let mut rng = rand::rng();
    rng.random_range(0.0..1.0)
}

pub fn random_double_range(min: f64, max: f64) -> f64 {
    let mut rng = rand::rng();
    rng.random_range(min..max)
}
