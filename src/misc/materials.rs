use std::fmt::Debug;

use crate::geometry::HitRecord;
use crate::m::rand_f;
use crate::ray::Ray;
use crate::vectors::{Color3, Colors, Vec3};

pub trait Material {
    fn scatter(self: &Self, ray_in: &Ray, hit_record: &HitRecord, attenuation: &mut Color3, scattered: &mut Ray) -> bool;
}

#[derive(Debug, Copy, Clone)]
pub struct Empty {}

impl Empty {
    pub fn get() -> Empty {
        Empty {}
    }
}

impl Material for Empty {
    fn scatter(self: &Self, _ray_in: &Ray, _hit_record: &HitRecord, _attenuation: &mut Color3, _scattered: &mut Ray) -> bool {
        false
    }
}


#[derive(Debug, Copy, Clone)]
pub struct Lambertian {
    albedo: Color3
}

impl Lambertian {
    pub fn new(c: &Color3) -> Lambertian {
        Lambertian { albedo: c.clone() }
    }
    pub fn new_f(r: f64, g: f64, b: f64) -> Lambertian {
        Lambertian { albedo: Vec3::new(r, g, b) }
    }

    pub fn rand() -> Lambertian {
        Lambertian { albedo: Color3::random() * Color3::random() }
    }
}


impl Material for Lambertian {
    fn scatter(self: &Self, _ray_in: &Ray, hit_record: &HitRecord, attenuation: &mut Color3, scattered: &mut Ray) -> bool {
        let mut scatter_direction = hit_record.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }

        scattered.origin = hit_record.p;
        scattered.direction = scatter_direction;
        attenuation[Colors::R] = self.albedo[Colors::R];
        attenuation[Colors::G] = self.albedo[Colors::G];
        attenuation[Colors::B] = self.albedo[Colors::B];
        true
    }
}


#[derive(Debug, Copy, Clone)]
pub struct Metal {
    albedo: Color3,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: &Vec3, fuzz: f64) -> Metal {
        let f = if fuzz > 1.0 {
            1.0
        } else {
            fuzz
        };
        Metal { albedo: albedo.clone(), fuzz: f }
    }

    pub fn new_f(r: f64, g: f64, b: f64, fuzz: f64) -> Metal {
        let f = if fuzz > 1.0 {
            1.0
        } else {
            fuzz
        };
        Metal { albedo: Vec3::new(r, g, b), fuzz: f }
    }

    pub fn rand() -> Metal {
        Metal::new(&(Color3::random() * Color3::random()), rand_f())
    }
}

impl Material for Metal {
    fn scatter(self: &Self, ray_in: &Ray, hit_record: &HitRecord, attenuation: &mut Color3, scattered: &mut Ray) -> bool {
        let reflected = Vec3::reflect(ray_in.direction.unit_vec(), hit_record.normal);
        scattered.origin = hit_record.p;
        scattered.direction = reflected + self.fuzz * Vec3::rand_in_unit_sphere();
        attenuation[Colors::R] = self.albedo[Colors::R];
        attenuation[Colors::G] = self.albedo[Colors::G];
        attenuation[Colors::B] = self.albedo[Colors::B];

        return scattered.direction.dot(hit_record.normal) > 0.0;
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Dielectric {
    ir: f64
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Dielectric {
        Dielectric { ir: index_of_refraction }
    }
}

impl Material for Dielectric {
    fn scatter(self: &Self, ray_in: &Ray, hit_record: &HitRecord, attenuation: &mut Color3, scattered: &mut Ray) -> bool {
        attenuation[Colors::R] = 1.0;
        attenuation[Colors::G] = 1.0;
        attenuation[Colors::B] = 1.0;

        let refraction_ratio = if hit_record.font_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = ray_in.direction.unit_vec();

        let cos_theta = -unit_direction.dot(hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let can_refract = refraction_ratio * sin_theta > 1.0;

        let dir = if can_refract || reflectance(cos_theta, refraction_ratio) > rand_f() {
            Vec3::reflect(unit_direction, hit_record.normal)
        } else {
            Vec3::refract(unit_direction, hit_record.normal, refraction_ratio)
        };

        scattered.origin = hit_record.p;
        scattered.direction = dir;
        true
    }
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * ((1.0 - cosine).powi(5))
}