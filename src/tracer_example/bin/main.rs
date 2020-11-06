use std::f64::consts::PI;
use std::io::{self, Stdout, Write};
use std::rc::Rc;

use misc::camera::Camera;
use misc::geometry::{HittableList, Sphere};
use misc::m::{rand_f, rand_f_mm};
use misc::materials::{Dielectric, Lambertian, Metal};
use misc::ppm::write_color_avg;
use misc::rays::ray_color;
use misc::vectors::{Color3, Point3};

fn main() {
    let aspect_ratio: f64 = 5.0 / 4.0;

    let image_width: u32 = 1200;
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;

    let samples_per_px: u32 = 500;
    let max_depth: u32 = 100;

    let lookfrom = Point3::new(13.0, 9.0, 7.0);

    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Point3::new(0.0, 1.0, 0.0);
    let vfov = 20.0;
    let apperture = 0.1;
    let focus_dist = 10.0;

    let camera = Camera::new(lookfrom, lookat, vup, vfov, aspect_ratio, apperture, focus_dist);

    let world = random_world();

    let stdout: &mut Stdout = &mut io::stdout();
    let stderr = &mut io::stderr();

    write!(stdout, "P3\n{} {}\n255\n", image_width, image_height).unwrap();

    for j in (1..image_height - 1).rev() {
        writeln!(stderr, "Scanlines remaining {}", j).unwrap();
        for i in 0..image_width {
            let mut color_px = Color3::zero();

            for _s in 0..samples_per_px {
                let u = (i as f64 + rand_f()) / (image_width as f64 - 1.0);
                let v = (j as f64 + rand_f()) / (image_height as f64 - 1.0);
                let r = camera.get_ray(u, v);
                color_px += ray_color(&r, &world, max_depth);
            }
            write_color_avg(stdout, &color_px, samples_per_px);
        }
    }
}


pub fn random_world() -> HittableList {
    let mut world = HittableList::empty();

    let ground_material = Rc::new(Lambertian::new_f(0.5, 0.5, 0.5));
    let ground_sphere = Rc::new(Sphere::new_f(0.0, -1000.0, 0.0, 1000.0, ground_material.clone()));
    world.add(ground_sphere);

    let dielectric = Rc::new(Dielectric::new(1.5));

    let p = Point3::new(4.0, 0.2, 0.0);
    for a in -30..30 {
        for b in -30..30 {
            let choose_mat = rand_f();
            let center = Point3::new(a as f64 + 0.9 * rand_f(), 0.2, b as f64 + 0.9 * rand_f());

            if (center - p).length() > 0.9 {
                if choose_mat < 0.6 {
                    let albedo = Color3::random() * Color3::random();
                    let sphere_mat = Rc::new(Lambertian::new(&albedo));
                    world.add(Rc::new(Sphere::new(&center, 0.2, sphere_mat)))
                } else if choose_mat < 0.85 {
                    let albedo = Color3::random_mm(0.5, 1.0);
                    let fuzz = rand_f_mm(0.0, 0.5);
                    let sphere_mat = Rc::new(Metal::new(&albedo, fuzz));
                    world.add(Rc::new(Sphere::new(&center, 0.2, sphere_mat)))
                } else {
                    world.add(Rc::new(Sphere::new(&center, 0.2, dielectric.clone())))
                }
            }
        }
    }


    world.add(Rc::new(Sphere::new_f(0.0, 1.0, 0.0, 1.0, dielectric.clone())));

    let material2 = Rc::new(Lambertian::rand());
    world.add(Rc::new(Sphere::new_f(-4.0, 1.0, 0.0, 1.0, material2)));

    let material3 = Rc::new(Metal::rand());
    world.add(Rc::new(Sphere::new_f(4.0, 1.0, 0.0, 1.0, material3)));

    let material4 = Rc::new(Lambertian::rand());
    world.add(Rc::new(Sphere::new_f(0.0, 1.0, 4.0, 1.0, material4)));

    world.add(Rc::new(Sphere::new_f(0.0, 1.0, -4.0, 1.0, dielectric.clone())));

    let material6 = Rc::new(Lambertian::rand());
    world.add(Rc::new(Sphere::new_f(4.0, 1.0, -4.0, 1.0, material6)));

    let material7 = Rc::new(Metal::rand());
    world.add(Rc::new(Sphere::new_f(-4.0, 1.0, 4.0, 1.0, material7)));

    world.add(Rc::new(Sphere::new_f(-4.0, 1.0, 4.0, 1.0, dielectric.clone())));

    let material9 = Rc::new(Metal::rand());
    world.add(Rc::new(Sphere::new_f(4.0, 1.0, -4.0, 1.0, material9)));


    world
}