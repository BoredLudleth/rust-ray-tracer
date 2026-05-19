use crate::rtweekend::INFINITY;

#[derive(Debug, Clone, Copy)]
pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    pub fn new() -> Self {
        Self {
            min: INFINITY,
            max: -INFINITY,
        }
    }
    
    pub fn from(min: f32, max: f32) -> Self {
        Self { min, max }
    }
    
    pub fn size(&self) -> f32 {
        self.max - self.min
    }
    
    pub fn contains(&self, x: f32) -> bool {
        self.min <= x && x <= self.max
    }
    
    pub fn surrounds(&self, x: f32) -> bool {
        self.min < x && x < self.max
    }
    
    pub fn clamp(&self, x: f32) -> f32 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }
    
    pub fn empty() -> Self {
        Self {
            min: INFINITY,
            max: -INFINITY,
        }
    }
    
    pub fn universe() -> Self {
        Self {
            min: -INFINITY,
            max: INFINITY,
        }
    }
}

impl Default for Interval {
    fn default() -> Self {
        Self::new()
    }
}