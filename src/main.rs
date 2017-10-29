extern crate num;
extern crate image;
extern crate palette;
extern crate pbr;

use num::Complex;
use image::{ RgbaImage, ImageBuffer, Rgba };
use std::path::Path;
use palette::{ Rgb, Gradient, IntoColor };
use pbr::ProgressBar;

pub struct Fractal {
    size: u32, // size of canvas
    startz: Complex<f64>, // starting value of z, z = 0 + 0i for mandelbrot
    num_iters: u32, // number of iterations to try
    image_buffer: RgbaImage,
    palette: Vec<Rgb>,
}

impl Fractal {

    const BLACK: Rgba<u8> = Rgba { data: [0, 0, 0, 255]};

    fn new(size: u32, startz: Complex<f64>, num_iters: u32) -> Fractal {
        let gradient = Gradient::with_domain(vec![
            (0.0,    Rgb::new_u8(0, 7, 100)),
            (0.16,   Rgb::new_u8(32, 107, 203)),
            (0.42,   Rgb::new_u8(237, 255, 255)),
            (0.6425, Rgb::new_u8(255, 170, 0)),
            (0.8575, Rgb::new_u8(0, 2, 0)),
        ]);

        // make palette
        let mut palette: Vec<Rgb> = Vec::new();
        for c in gradient.take(255) {
            palette.push(c.into_rgb());
        }

        Fractal {
            size: size,
            startz: startz,
            num_iters: num_iters,
            image_buffer: ImageBuffer::new(size, size),
            palette: palette,
        }

    }

    fn generate(&mut self, path: &str) {
        let mut progress = ProgressBar::new(self.size as u64);
        progress.show_time_left = false;
        progress.show_speed = false;
        let inc = 3.0 / (self.size as f64);
        let xshift = (2 * self.size / 3) as i32;
        let yshift = (self.size / 2) as i32;
        for x in 0..self.size {
            let i32x = x as i32;
            let re = ((i32x - xshift) as f64) * inc;
            for y in 0..self.size {
                let i32y = y as i32;
                let im = ((i32y - yshift) as f64) * inc;
                let c = Complex::new(re, im);
                // print!("{}, {}                                               \r", re, im);
                let iter = Fractal::mandelbrot(self.startz, c, self.num_iters);
                if iter != -1. {
                    let iter = (255. * iter)/(self.num_iters as f32);
                    let rgb = self.palette[iter as usize];
                    let (r, g, b) = ((rgb.red * 255.) as u8, (rgb.green * 255.) as u8, (rgb.blue * 255.) as u8);
                    self.image_buffer.put_pixel(x, y, Rgba { data: [r, g, b, 255]}); // outside
                } else {
                    self.image_buffer.put_pixel(x, y, Fractal::BLACK); // inside
                }
            }
            progress.inc();
        }
        progress.finish_println("Finished Generating! Saving result.");
        let path = &format!("{}-i{}-r{}.png", path, self.num_iters, self.size);
        self.image_buffer.save(&Path::new(path)).unwrap();
        println!("");
        println!("Saved image to {}", path);
    }

    fn mandelbrot(z: Complex<f64>, c: Complex<f64>, num_iters: u32) -> f32 {
        if num_iters <= 0 {
            return -1.;
        }

        if z.norm() > 2.0 {
            return num_iters as f32;
        }

        let z = z.powf(2.0) + c;
        Fractal::mandelbrot(z, c, num_iters-1)
    }
}


fn main() {
    let size = 2000;

    for i in vec![25,35,45] {
        let mut fractal = Fractal::new(size, Complex::new(0.0, 0.0), i);
        fractal.generate("test");
    }
}
