mod vec3;
mod ray;
mod color;
mod rtweekend;
mod interval;
mod hittable;
mod sphere;
mod hittable_list;
mod camera;
mod material;

use std::sync::Arc;
use vec3::Point3;
use color::Color;
use hittable_list::HittableList;
use sphere::Sphere;
use camera::Camera;
use material::{Lambertian, Metal};

fn main() {
    let mut world = HittableList::new();
    
    // without fuzziness
    let material_ground = Arc::new(Lambertian::new(Color::from(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Color::from(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Metal::new(Color::from(0.8, 0.8, 0.8), 0.0));
    let material_right = Arc::new(Metal::new(Color::from(0.8, 0.6, 0.2), 0.0));
    
    // with fuzziness
    // let material_ground = Arc::new(Lambertian::new(Color::from(0.8, 0.8, 0.0)));
    // let material_center = Arc::new(Lambertian::new(Color::from(0.1, 0.2, 0.5)));
    // let material_left = Arc::new(Metal::new(Color::from(0.8, 0.8, 0.8), 0.3));
    // let material_right = Arc::new(Metal::new(Color::from(0.8, 0.6, 0.2), 1.0));
    
    world.add(Box::new(Sphere::new(Point3::from(0.0, -100.5, -1.0), 100.0, material_ground)));
    world.add(Box::new(Sphere::new(Point3::from(0.0, 0.0, -1.2), 0.5, material_center)));
    world.add(Box::new(Sphere::new(Point3::from(-1.0, 0.0, -1.0), 0.5, material_left)));
    world.add(Box::new(Sphere::new(Point3::from(1.0, 0.0, -1.0), 0.5, material_right)));
    
    let mut cam = Camera::new();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    
    cam.render(&world);
}