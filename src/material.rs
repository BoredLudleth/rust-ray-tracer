use crate::vec3::{self, random_unit_vector, reflect};
use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::color::Color;

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut scatter_direction = rec.normal + random_unit_vector();
        
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        
        *scattered = Ray::from(rec.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut reflected = reflect(r_in.direction(), rec.normal);
        reflected = vec3::unit_vector(reflected) + (self.fuzz * random_unit_vector());
        *scattered = Ray::from(rec.p, reflected);
        *attenuation = self.albedo;
        vec3::dot(&scattered.direction(), &rec.normal) > 0.0
    }
}
