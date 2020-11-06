use crate::m::degree_to_rad;
use crate::ray::Ray;
use crate::vectors::{Point3, Vec3, Axis};

pub struct Camera {
    origin: Point3,
    ll_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    w: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(lookfrom: Point3,
               lookat: Point3,
               vup: Vec3,
               vfov: f64,
               aspect_ratio: f64,
               apperture: f64,
               focus_dist: f64) -> Camera {
        let theta = degree_to_rad(vfov);
        let h = (theta / 2.0).tan();

        let viewport_height: f64 = 2.0 * h;
        let viewport_width: f64 = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit_vec();
        let u = vup.cross(w).unit_vec();
        let v = w.cross(u);


        let origin: Vec3 = lookfrom;
        let horizontal: Vec3 = focus_dist * viewport_width * u;
        let vertical: Vec3 = focus_dist * viewport_height * v;
        let ll_corner: Vec3 = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;
        let lens_radius = apperture / 2.0;

        Camera { origin, ll_corner, horizontal, vertical, w, u, v, lens_radius }
    }

    pub fn get_ray(self: &Self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.u * rd[Axis::X] + self.v * rd[Axis::Y];
        Ray::new(&(self.origin + offset),
                 &(self.ll_corner + s * self.horizontal + t * self.vertical - self.origin - offset))
    }
}