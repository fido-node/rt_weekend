use std::f64::MAX;

use crate::geometry::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vectors::{Axis, Color3, Vec3};

pub fn ray_color(r: &Ray, world: &dyn Hittable, depth: u32) -> Color3 {
    if depth <= 0 {
        return Color3::zero();
    }

    let mut hit_record = HitRecord::empty();
    let is_hit = world.hit(r, 0.001, MAX, &mut hit_record);

    if is_hit {
        let mut scattered = Ray::new(&Vec3::zero(), &Vec3::zero());
        let mut attenuation = Color3::zero();
        let mat_hit = hit_record.mat.scatter(r, &hit_record, &mut attenuation, &mut scattered);
        if mat_hit {
            return attenuation * ray_color(&scattered, world, depth - 1);
        } else {
            Color3::zero();
        }
        //
        // let target = hit_record.p + hit_record.normal + Vec3::random_unit_vector();
        //
        // return 0.5 * ray_color(&Ray::new(&hit_record.p, &(target - hit_record.p)), world, depth - 1);
    }

    let unit_direction = r.direction.unit_vec();
    let t = 0.5 * (unit_direction[Axis::Y] + 1.0);
    (1.0 - t) * Color3::new(1.0, 1.0, 1.0) + t * Color3::new(0.5, 0.7, 1.0)
}