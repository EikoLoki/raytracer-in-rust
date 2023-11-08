use rand::prelude::*;

// Constants
pub const INFINITY: f64 = f64::INFINITY;
const PI: f64 = std::f64::consts::PI;

// Utility Functions
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

// Random
pub fn random() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

pub fn random_in_range(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}

// Interval
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn from(_min: f64, _max: f64) -> Self {
        Self {
            min: _min,
            max: _max,
        }
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            return self.min;
        }
        if x > self.max {
            return self.max;
        }

        x
    }
}

pub const EMPTY: Interval = Interval {
    min: INFINITY,
    max: -INFINITY,
};
pub const UNIVERSE: Interval = Interval {
    min: -INFINITY,
    max: INFINITY,
};
