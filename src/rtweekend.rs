use rand::Rng;

pub const INFINITY: f32 = f32::INFINITY;
pub const PI: f32 = 3.1415926535897932385;

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

pub fn random_double() -> f32 {
    rand::thread_rng().r#gen()
}

pub fn random_double_range(min: f32, max: f32) -> f32 {
    rand::thread_rng().gen_range(min..max)
}