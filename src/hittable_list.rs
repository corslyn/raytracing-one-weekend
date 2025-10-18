use crate::{
    hittable::*,
    interval::Interval,
    ray::*,
    vec3::{Point3, Vec3},
};
use std::sync::Arc;

#[derive(Clone)]
pub struct HittableList {
    objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList { objects: vec![] }
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            let mut temp_rec = HitRecord::default();

            if object
                .as_ref()
                .hit(r, Interval::new(ray_t.min, closest_so_far), &mut temp_rec)
            {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }
        hit_anything
    }
}
