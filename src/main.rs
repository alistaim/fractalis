mod color;

use crate::color::{hsl_to_rgb, Color};
use image::{ImageBuffer, RgbaImage, Progress};
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use std::path::Path;
use std::time::Instant;
use std::sync::{Arc, Mutex};
use rayon::prelude::*;

const PX: f64 = -0.5557506;
const PY: f64 = -0.5556003;
const PH: f64 = 0.0000000007;

// Quality
const IMG_WIDTH: u32 = 1024;
const IMG_HEIGHT: u32 = 768;
const MAX_ITER: i32 = 5000;
const SAMPLES: u8 = 128;

const RATIO: f64 = IMG_WIDTH as f64 / IMG_HEIGHT as f64;

pub fn main() {
    println!("Generating Image....");

    let path = Path::new(r"./output.png");
    let image = Arc::new(Mutex::new(ImageBuffer::new(IMG_WIDTH, IMG_HEIGHT)));

    let start = Instant::now();
    render(&image);
    let duration = start.elapsed();

    println!("Render time = {:.2?}", duration);
    match image.lock().unwrap().save(path) {
        Ok(_) => println!("Image saved successfully to {}", path.display()),
        Err(_) => eprint!("Error writing image file {}", path.display()),
    };
}

pub fn render(image: &Arc<Mutex<RgbaImage>>) {
    for y in 0..IMG_HEIGHT {
        (0..IMG_WIDTH).into_par_iter().for_each(|x| {
            let local_image = image.clone();
            let mut rng = Pcg64Mcg::new(0xcafebabeffff);
            let mut r: i32 = 0;
            let mut g: i32 = 0;
            let mut b: i32 = 0;

            for _ in 0..SAMPLES {
                let nx: f64 = PH * RATIO * ((x as f64 + rng.gen::<f64>()) / IMG_WIDTH as f64) + PX;
                let ny: f64 = PH * ((y as f64) + rng.gen::<f64>()) / (IMG_HEIGHT as f64) + PY;
                let c = paint(mandelbrot_iter(nx, ny, MAX_ITER));

                r = r + c.r as i32;
                g = g + c.g as i32;
                b = b + c.b as i32;
            }

            let cr = (r as f64 / SAMPLES as f64) as u8;
            let cg = (g as f64 / SAMPLES as f64) as u8;
            let cb = (b as f64 / SAMPLES as f64) as u8;

            local_image.lock().expect("Could Not get lock on image buffer mutex")
                .put_pixel(x, y, image::Rgba([cr, cg, cb, 255]));
        });
        printf("{}",
    }
}

fn paint((r, n): (f64, i32)) -> Color {
    let inside_set = Color::new(255, 255, 255, 255);

    if r > 4.0 {
        return hsl_to_rgb(n as f64 / 800.0 * r, 1.0, 0.5);
    }

    inside_set
}

fn mandelbrot_iter(px_var: f64, py_var: f64, max_iter_var: i32) -> (f64, i32) {
    let mut x: f64 = 0.0;
    let mut y: f64 = 0.0;
    let mut xx: f64 = 0.0;
    let mut yy: f64 = 0.0;
    let mut xy;

    for i in 0..max_iter_var {
        xx = x * x;
        yy = y * y;
        xy = x * y;
        if xx + yy > 4.0 {
            return (xx + yy, i);
        }

        x = xx - yy + px_var;
        y = 2.0 * xy + py_var;
    }

    (xx + yy, MAX_ITER)
}
