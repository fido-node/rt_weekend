use std::io::{Stdout, Write};

use crate::m::clamp;
use crate::vectors::{Color3, Colors};

pub fn write_color(std_str: &mut Stdout, v: &Color3) -> () {
    let ir = (255.999 * v[Colors::R]) as i32;
    let ig = (255.999 * v[Colors::G]) as i32;
    let ib = (255.999 * v[Colors::B]) as i32;
    writeln!(std_str, "{} {} {}", ir, ig, ib).unwrap();
}


pub fn write_color_avg(std_str: &mut Stdout, v: &Color3, samples_per_pixel: u32) -> () {
    let mut r = v[Colors::R];
    let mut g = v[Colors::G];
    let mut b = v[Colors::B];

    let scale = 1.0 / samples_per_pixel as f64;

    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();


    let ir = (256.0 * clamp(r, 0.0, 0.999)) as i32;
    let ig = (256.0 * clamp(g, 0.0, 0.999)) as i32;
    let ib = (256.0 * clamp(b, 0.0, 0.999)) as i32;
    writeln!(std_str, "{} {} {}", ir, ig, ib).unwrap();
}
