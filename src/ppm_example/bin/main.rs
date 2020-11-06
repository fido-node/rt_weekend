use std::io::{self, Write, Stdout};

use misc::vectors::Vec3;
use misc::ppm::write_color;

const IMAGE_WIDTH: u32 = 256;
const IMAGE_HEIGHT: u32 = 256;

fn main() {
    let stdout: &mut Stdout = &mut io::stdout();
    let stderr = &mut io::stderr();

    write!(stdout, "P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT).unwrap();

    for y in (1..IMAGE_HEIGHT - 1).rev() {
        writeln!(stderr, "Scanlines remaining {}", y).unwrap();
        for x in 0..IMAGE_WIDTH {

            let color = Vec3::new(x as f64 / (IMAGE_WIDTH - 1) as f64,
                                  y as f64 / (IMAGE_HEIGHT - 1) as f64,
                                  0.25);
            write_color(stdout, &color)
        }
    }
}
