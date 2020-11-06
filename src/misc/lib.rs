pub mod vectors;
pub mod ppm;
pub mod ray;
pub mod rays;
pub mod camera;
pub mod geometry;
pub mod materials;


pub mod m {
    use std::f64::consts::PI;

    use rand::prelude::*;

    pub fn degree_to_rad(degrees: f64) -> f64 {
        degrees * PI / 180.0
    }

    pub fn rand_f() -> f64 {
        let mut rng = rand::thread_rng();
        rng.gen_range(0.0, 1.0)
    }

    pub fn rand_f_mm(min: f64, max: f64) -> f64 {
        let mut rng = rand::thread_rng();
        rng.gen_range(min, max)
    }

    pub fn clamp(v: f64, min: f64, max: f64) -> f64 {
        if v < min { return min; }
        if v > max { return max; }
        return v;
    }
}