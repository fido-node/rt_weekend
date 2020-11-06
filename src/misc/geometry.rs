use std::rc::Rc;
use std::vec::Vec;

use crate::materials::{Empty, Material};
use crate::ray::Ray;
use crate::vectors::{Point3, Vec3};

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Rc<dyn Material>,
    pub t: f64,
    pub font_face: bool,
}


impl HitRecord {
    pub fn empty() -> HitRecord {
        HitRecord { p: Point3::new(0.0, 0.0, 0.0), normal: Vec3::new(0.0, 0.0, 0.0), mat: Rc::new(Empty::get()), t: 0.0, font_face: false }
    }

    pub fn set_font_face(self: &mut Self, r: &Ray, outward_normal: Vec3) -> () {
        self.font_face = r.direction.dot(outward_normal) < 0.0;
        self.normal = if self.font_face { outward_normal.clone() } else { -outward_normal };
    }
}

pub trait Hittable {
    fn hit(self: &Self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool;
}


pub struct Sphere {
    cen: Point3,
    r: f64,
    m: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(c: &Point3, r: f64, m: Rc<dyn Material>) -> Sphere {
        Sphere { cen: c.clone(), r, m }
    }

    pub fn new_f(x: f64, y: f64, z: f64, r: f64, m: Rc<dyn Material>) -> Sphere {
        Sphere { cen: Vec3::new(x, y, z), r, m }
    }
}

impl Hittable for Sphere {
    fn hit(self: &Sphere, r: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let oc = r.origin - self.cen;
        let a = r.direction.length_squared();
        let half_b = oc.dot(r.direction);
        let c = oc.length_squared() - self.r * self.r;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return false;
            }
        }

        hit_record.t = root;
        hit_record.p = r.at(root);
        let outward_normal = (hit_record.p - self.cen) / self.r;
        hit_record.set_font_face(r, outward_normal);
        hit_record.mat = self.m.clone();
        true
    }
}

pub struct HittableList {
    elements: Vec<Rc<dyn Hittable>>
}

impl HittableList {
    pub fn empty() -> HittableList {
        HittableList { elements: Vec::new() }
    }

    pub fn add(self: &mut Self, h: Rc<dyn Hittable>) -> () {
        self.elements.push(h);
    }

    pub fn clear(self: &mut Self) -> () {
        self.elements.clear();
    }
}

impl Hittable for HittableList {
    fn hit(self: &Self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let mut hit_smth = false;
        let mut tmp_hit = HitRecord::empty();
        let mut closest_so_far = t_max;

        let iter = self.elements.iter();

        for element in iter {
            if element.hit(ray, t_min, closest_so_far, &mut tmp_hit) {
                hit_smth = true;
                closest_so_far = tmp_hit.t;

                hit_record.normal = tmp_hit.normal;
                hit_record.p = tmp_hit.p;
                hit_record.font_face = tmp_hit.font_face;
                hit_record.t = tmp_hit.t;
                hit_record.mat.clone_from(&tmp_hit.mat);
            }
        }

        return hit_smth;
    }
}