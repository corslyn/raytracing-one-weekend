use crate::{
    hittable::*,
    ray::{self, Ray},
    rtweekend::random_double,
    vec3::{Color, Vec3, dot, random_unit_vector, reflect, refract, unit_vector},
};

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        return true;
    }
}

impl Default for Lambertian {
    fn default() -> Self {
        Self {
            albedo: Color::new(0.5, 0.5, 0.5),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        Metal { albedo, fuzz }
    }
}
impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut reflected = reflect(r_in.direction(), rec.normal);
        reflected = unit_vector(reflected) + self.fuzz * random_unit_vector();
        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;
        return true;
    }
}

pub struct Dielectic {
    refraction_index: f64,
}

impl Dielectic {
    pub fn new(refraction_index: f64) -> Dielectic {
        Dielectic { refraction_index }
    }

    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}

impl Material for Dielectic {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);

        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = unit_vector(r_in.direction());
        let cos_theta = 1.0_f64.min(dot(-unit_direction, rec.normal));
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = if (ri * sin_theta) > 1.0 { true } else { false };

        let direction = if cannot_refract
            || Dielectic::reflectance(cos_theta, self.refraction_index) > random_double()
        {
            reflect(unit_direction, rec.normal)
        } else {
            refract(unit_direction, rec.normal, ri)
        };

        *scattered = Ray::new(rec.p, direction);
        return true;
    }
}
