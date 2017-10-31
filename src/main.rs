// extern crate num;
// extern crate image;
// extern crate palette;
// extern crate pbr;
#[macro_use]
extern crate glium;

// use num::Complex;
// use image::{ RgbaImage, ImageBuffer, Rgba };
// use std::path::Path;
// use palette::{ Rgb, Gradient, IntoColor };
// use pbr::ProgressBar;
use glium::{ Display, Surface };
use glium::glutin::{ ContextBuilder, ElementState, EventsLoop, Event, WindowBuilder, WindowEvent, KeyboardInput, VirtualKeyCode };

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);

// pub struct Fractal {
//     size: u32, // size of canvas
//     startz: Complex<f64>, // starting value of z, z = 0 + 0i for mandelbrot
//     num_iters: u32, // number of iterations to try
//     image_buffer: RgbaImage,
//     palette: Vec<Rgb>,
// }

// impl Fractal {

//     const BLACK: Rgba<u8> = Rgba { data: [0, 0, 0, 255]};

//     fn new(size: u32, startz: Complex<f64>, num_iters: u32) -> Fractal {
//         let gradient = Gradient::with_domain(vec![
//             (0.0,    Rgb::new_u8(0, 7, 100)),
//             (0.16,   Rgb::new_u8(32, 107, 203)),
//             (0.42,   Rgb::new_u8(237, 255, 255)),
//             (0.6425, Rgb::new_u8(255, 170, 0)),
//             (0.8575, Rgb::new_u8(0, 2, 0)),
//         ]);

//         // make palette
//         let mut palette: Vec<Rgb> = Vec::new();
//         for c in gradient.take(255) {
//             palette.push(c.into_rgb());
//         }

//         Fractal {
//             size: size,
//             startz: startz,
//             num_iters: num_iters,
//             image_buffer: ImageBuffer::new(size, size),
//             palette: palette,
//         }

//     }

//     fn generate(&mut self, path: &str) {
//         let mut progress = ProgressBar::new(self.size as u64);
//         progress.show_time_left = false;
//         progress.show_speed = false;
//         let inc = 3.0 / (self.size as f64);
//         let xshift = (2 * self.size / 3) as i32;
//         let yshift = (self.size / 2) as i32;
//         for x in 0..self.size {
//             let i32x = x as i32;
//             let re = ((i32x - xshift) as f64) * inc;
//             for y in 0..self.size {
//                 let i32y = y as i32;
//                 let im = ((i32y - yshift) as f64) * inc;
//                 let c = Complex::new(re, im);
//                 // print!("{}, {}                                               \r", re, im);
//                 let iter = Fractal::mandelbrot(self.startz, c, self.num_iters);
//                 if iter != -1. {
//                     let iter = (255. * iter)/(self.num_iters as f32);
//                     let rgb = self.palette[iter as usize];
//                     let (r, g, b) = ((rgb.red * 255.) as u8, (rgb.green * 255.) as u8, (rgb.blue * 255.) as u8);
//                     self.image_buffer.put_pixel(x, y, Rgba { data: [r, g, b, 255]}); // outside
//                 } else {
//                     self.image_buffer.put_pixel(x, y, Fractal::BLACK); // inside
//                 }
//             }
//             progress.inc();
//         }
//         progress.finish_println("Finished Generating! Saving result.");
//         let path = &format!("{}-i{}-r{}.png", path, self.num_iters, self.size);
//         self.image_buffer.save(&Path::new(path)).unwrap();
//         println!("");
//         println!("Saved image to {}", path);
//     }

//     fn mandelbrot(z: Complex<f64>, c: Complex<f64>, num_iters: u32) -> f32 {
//         if num_iters <= 0 {
//             return -1.;
//         }

//         if z.norm() > 2.0 {
//             return num_iters as f32;
//         }

//         let z = z.powf(2.0) + c;
//         Fractal::mandelbrot(z, c, num_iters-1)
//     }
// }


fn main() {
    // let size = 2000;

    // for i in vec![25,35,45] {
    //     let mut fractal = Fractal::new(size, Complex::new(0.0, 0.0), i);
    //     fractal.generate("test");
    // }

    let mut screen_size = [2048, 1534];

    // create window
    let mut events_loop = EventsLoop::new();
    let window = WindowBuilder::new()
        .with_title("Mandelbrot".to_string())
        .with_dimensions(screen_size[0], screen_size[1]);
    let context = ContextBuilder::new();
    let display = Display::new(window, context, &events_loop).unwrap();

    // compile the shaders
    let program = glium::Program::from_source(
        &display,
        include_str!("shaders/mandelbrot.vert"),
        include_str!("shaders/mandelbrot.frag"),
        None
    ).unwrap();

    // render 2 triangles covering the screen
    let vertices = [
        Vertex { position: [-1.0, 1.0]},
        Vertex { position: [1.0, 1.0]},
        Vertex { position: [-1.0, -1.0]},

        Vertex { position: [-1.0, -1.0]},
        Vertex { position: [1.0, 1.0]},
        Vertex { position: [1.0, -1.0]},
    ];

    let vertex_buffer = glium::VertexBuffer::new(&display, &vertices).unwrap();

    let mut max_iter: u32 = 50;
    let mut offset_x: f32 = -1.0;
    let mut offset_y: f32 = -1.0;
    let mut zoom: f32 = f32::log(1.0, 2.0);
    let mut zoom_rate: f32 = 0.1;
    let mut pan_rate: f32 = 0.01;
    let mut z: [f32; 2] = [-0.618033989, 0.00];

    let mut closed = false;
    while !closed {
        let mut target = display.draw();

        let zoomx = screen_size[0] as f32 * zoom.exp2();
        let zoomy = screen_size[1] as f32 * zoom.exp2();
        target.draw(
            &vertex_buffer,
            &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
            &program,
            &uniform! {
                start_z: z,
                max_iter: max_iter,
                screen_x: zoomx,
                screen_y: zoomy,
                offset_x: offset_x - (screen_size[0] as f32 / zoomx),
                offset_y: offset_y - (screen_size[0] as f32 / zoomx)},
            &Default::default()
        ).unwrap();

        target.finish().unwrap();

        events_loop.poll_events(|ev| {
            match ev {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::Closed => closed = true,
                    WindowEvent::Resized(x, y) => { screen_size[0] = x; screen_size[1] = y}
                    WindowEvent::KeyboardInput { input, .. } => match input {
                        KeyboardInput { virtual_keycode, state: ElementState::Pressed, .. } => match virtual_keycode {
                            Some(VirtualKeyCode::Up) => z[0] += pan_rate * 0.1,
                            Some(VirtualKeyCode::Down) => z[0] -= pan_rate * 0.1,
                            Some(VirtualKeyCode::Left) => z[1] += pan_rate * 0.1,
                            Some(VirtualKeyCode::Right) => z[1] -= pan_rate * 0.1,
                            Some(VirtualKeyCode::W) => offset_y += pan_rate,
                            Some(VirtualKeyCode::S) => offset_y -= pan_rate,
                            Some(VirtualKeyCode::A) => offset_x -= pan_rate,
                            Some(VirtualKeyCode::D) => offset_x += pan_rate,
                            Some(VirtualKeyCode::T) => {max_iter += 1; println!("max_iter: {}", max_iter)},
                            Some(VirtualKeyCode::G) => {max_iter -= 1; println!("max_iter: {}", max_iter)},
                            Some(VirtualKeyCode::E) => {
                                zoom += zoom_rate;
                                pan_rate = 0.1 / zoom.exp2();
                            },
                            Some(VirtualKeyCode::Q) => zoom -= zoom_rate,
                            Some(VirtualKeyCode::Space) => {
                                offset_x = -1.0; offset_y = -1.0; zoom = f32::log(1.0, 2.0);
                                pan_rate = 0.1; zoom_rate = 0.1; max_iter = 50;
                                z = [0.0, 0.0];
                            },
                            Some(VirtualKeyCode::Y) => {
                                zoom_rate += 0.1;
                                println!("zoom_rate: {}", zoom_rate);
                            },
                            Some(VirtualKeyCode::H) => {
                                zoom_rate -= 0.1;
                                println!("zoom_rate: {}", zoom_rate);
                            },
                            _ => (),
                        }
                        _ => (),
                    }

                    _ => (),
                },
                _ => ()
            }
        });
    }
}
