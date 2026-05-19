use crate::vec3::{self, Point3, Vec3};
use crate::ray::Ray;
use crate::color::{Color, write_color};
use crate::hittable::{Hittable, HitRecord};
use crate::interval::Interval;
use crate::rtweekend::{INFINITY, random_double};

pub struct Camera {
    pub aspect_ratio: f32,
    pub image_width: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    
    image_height: i32,
    pixel_samples_scale: f32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,
            image_height: 0,
            pixel_samples_scale: 0.0,
            center: Point3::new(),
            pixel00_loc: Point3::new(),
            pixel_delta_u: Vec3::new(),
            pixel_delta_v: Vec3::new(),
        }
    }
    
    pub fn render(&mut self, world: &dyn Hittable) {
        self.initialize();
        
        println!("P3\n{} {}\n255", self.image_width, self.image_height);
        
        for j in 0..self.image_height {
            eprint!("\rScanlines remaining: {}", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_color = Color::from(0.0, 0.0, 0.0);
                
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += self.ray_color(&r, self.max_depth, world);
                }
                
                write_color(self.pixel_samples_scale * pixel_color);
            }
        }
        
        eprintln!("\rDone.                 ");
    }
    
    fn initialize(&mut self) {
        self.image_height = (self.image_width as f32 / self.aspect_ratio) as i32;
        if self.image_height < 1 {
            self.image_height = 1;
        }
        
        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f32;
        self.center = Point3::from(0.0, 0.0, 0.0);
        
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f32 / self.image_height as f32);
        
        let viewport_u = Vec3::from(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::from(0.0, -viewport_height, 0.0);
        
        self.pixel_delta_u = viewport_u / self.image_width as f32;
        self.pixel_delta_v = viewport_v / self.image_height as f32;
        
        let viewport_upper_left = self.center
            - Vec3::from(0.0, 0.0, focal_length)
            - viewport_u / 2.0
            - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5;
    }
    
    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset_x = random_double() - 0.5;
        let offset_y = random_double() - 0.5;
        
        let pixel_sample = self.pixel00_loc
            + ((i as f32 + offset_x) * self.pixel_delta_u)
            + ((j as f32 + offset_y) * self.pixel_delta_v);
        
        let ray_direction = pixel_sample - self.center;
        Ray::from(self.center, ray_direction)
    }
    
    fn ray_color(&self, r: &Ray, depth: i32, world: &dyn Hittable) -> Color {
        if depth <= 0 {
            return Color::from(0.0, 0.0, 0.0);
        }
        
        let mut rec = HitRecord::new();
        
        if world.hit(r, Interval::from(0.001, INFINITY), &mut rec) {
            if let Some(mat) = &rec.mat {
                let mut scattered = Ray::new();
                let mut attenuation = Color::new();
                
                if mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
                    return attenuation * self.ray_color(&scattered, depth - 1, world);
                }
            }
            return Color::from(0.0, 0.0, 0.0);
        }
        
        let unit_direction = vec3::unit_vector(r.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);
        
        let white = Color::from(1.0, 1.0, 1.0);
        let blue = Color::from(0.5, 0.7, 1.0);
        
        white * (1.0 - a) + blue * a
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new()
    }
}