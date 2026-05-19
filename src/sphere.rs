use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;
use crate::vec3::{self, Point3};
use crate::interval::Interval;
use crate::material::Material;
use std::sync::Arc;

pub struct Sphere {
    center: Point3,
    radius: f32,
    mat: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32, mat: Arc<dyn Material>) -> Self {
        Self {
            center,
            radius: radius.max(0.0),
            mat,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let oc = self.center - r.origin();
        let a = r.direction().squared_length();
        let h = vec3::dot(&r.direction(), &oc);
        let c = oc.squared_length() - self.radius * self.radius;
        
        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        }
        
        let sqrtd = discriminant.sqrt();
        
        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }
        
        rec.t = root;
        rec.p = r.at(rec.t);
        
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        rec.mat = Some(self.mat.clone());
        
        true
    }
}