use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;
use crate::interval::Interval;

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }
    
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
    
    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Default for HittableList {
    fn default() -> Self {
        Self::new()
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;
        
        for object in &self.objects {
            let test_interval = Interval::from(ray_t.min, closest_so_far);
            if object.hit(r, test_interval, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }
        
        hit_anything
    }
}