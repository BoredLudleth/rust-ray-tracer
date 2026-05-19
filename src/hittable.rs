use crate::ray::Ray;
use crate::vec3::{self, Vec3, Point3};
use crate::interval::Interval;
use crate::material::Material;
use std::sync::Arc;
use std::fmt;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Option<Arc<dyn Material>>,
    pub t: f32,
    pub front_face: bool,
}

impl fmt::Debug for HitRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HitRecord")
            .field("p", &self.p)
            .field("normal", &self.normal)
            .field("mat", &self.mat.as_ref().map(|_| "Some(Material)"))
            .field("t", &self.t)
            .field("front_face", &self.front_face)
            .finish()
    }
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            p: Point3::new(),
            normal: Vec3::new(),
            mat: None,
            t: 0.0,
            front_face: true,
        }
    }
    
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = vec3::dot(&r.direction(), &outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

impl Default for HitRecord {
    fn default() -> Self {
        Self::new()
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}