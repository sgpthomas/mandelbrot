extern crate glium_text;
#[macro_use]
extern crate glium;
extern crate image;
extern crate pbr;
extern crate threadpool;

use glium::{ Surface };
use glium::backend::glutin::Display;
use glium::glutin::{ ContextBuilder, ElementState, EventsLoop, Event, WindowBuilder, WindowEvent, KeyboardInput, VirtualKeyCode, MouseButton };
use image::{ RgbImage, ImageBuffer, Rgb };
use image::imageops;
use std::path::Path;
use std::fs;
use pbr::ProgressBar;
use std::io::Write;
use std::io::stdout;
use threadpool::ThreadPool;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);


fn main() {

    let mut screen_size: [f32; 2] = [2048.0, 1534.0];

   // create window
    let mut events_loop = EventsLoop::new();
    let window = WindowBuilder::new()
        .with_title("Mandelbrot".to_string())
        .with_dimensions(screen_size[0] as u32, screen_size[1] as u32);
    let context = ContextBuilder::new();
    let display = Display::new(window, context, &events_loop).unwrap();

    // font stuff
    // let system = TextSystem::new(&display);

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

    let mut max_iter: i32 = 200;
    let mut center: [f64; 2] = [0.0, 0.0];
    let mut zoom: f32 = f32::ln(1.0);
    let mut zoom_rate: f32 = 0.1;
    let mut pan_rate: f64 = 0.1;
    let mut z: [f64; 2] = [0.0, 0.0];
    let mut mode: i32 = 0; // 0 = mandelbrot, 1 = julia
    let mut color_mode = 0;
    let mut mouse_x: f64 = 0.0;
    let mut mouse_y: f64 = 0.0;
    let mut anim_center: [f64; 2] = [0.0, 0.0];

    let mut closed = false;
    let mut draw = true;
    let mut anim = -1;
    let max_zoom = 32.;
    let mut progress = ProgressBar::new(1);

    // image processing thread
    let pool = ThreadPool::new(2);

    while !closed {

        if draw {
            let mut target = display.draw();
            target.draw(
                &vertex_buffer,
                &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                &program,
                &uniform! {
                    start_z: z,
                    max_iter: max_iter,
                    screen_size: screen_size,
                    zoom: zoom.exp(),
                    center: center,
                    mode: mode,
                    color_mode: color_mode,
                },
                &Default::default()
            ).unwrap();
            target.finish().unwrap();

            if anim >= 0 {
                let dir_str = format!("{}_{}", center[0], center[1]);
                let path = Path::new(&dir_str);
                if !path.exists() {
                    match fs::create_dir_all(path) {
                        Ok(_) => (),
                        Err(_) => { println!("Failed to create folder! Aborting animation!"); anim = -1 }
                    }
                }
                let path_str = format!("{}/a{}.png", dir_str, anim);

                let pixels: Vec<Vec<(u8, u8, u8, u8)>> = display.read_front_buffer();

                let mut img: RgbImage = ImageBuffer::new(pixels[0].len() as u32, pixels.len() as u32);
                for y in 0..pixels.len() {
                    for x in 0..pixels[y].len() {
                        let pix = pixels[y][x];
                        img.put_pixel(x as u32, y as u32, Rgb { data: [pix.0, pix.1, pix.2]});
                    }
                }
                pool.execute(move || {
                    let img = imageops::flip_vertical(&img);
                    let path = Path::new(&path_str);
                    img.save(&path).unwrap();
                });

                if zoom > max_zoom {
                    anim = -1;
                    progress.finish_println("Finished Zooming!\n");
                    print!("Waiting on {} jobs ... ", pool.active_count());
                    stdout().flush().unwrap();
                    pool.join();
                    println!("Done");
                } else {
                    progress.inc();
                    zoom += zoom_rate;
                    anim += 1;
                }
            } else {
                draw = false;
            }
        }

        if anim < 0 {
            events_loop.poll_events(|ev| {
                match ev {
                    Event::WindowEvent { event, .. } => match event {
                        WindowEvent::Closed => closed = true,
                        WindowEvent::Resized(x, y) => { draw = true; screen_size[0] = x as f32; screen_size[1] = y as f32}
                        WindowEvent::MouseMoved { position: (x, y), .. } =>  { mouse_x = x as f64; mouse_y = y; }
                        WindowEvent::MouseInput { button: MouseButton::Left, state: ElementState::Pressed, .. } => {
                            let zoom_factor: f64 = (2. / zoom) as f64;
                            let max_screen_size = f32::max(screen_size[0], screen_size[1]) as f64;

                            let mut offset_x: f64 = 0.0;
                            let mut offset_y: f64 = 0.0;
                            if (screen_size[0] as f64) == max_screen_size {
                                offset_y = ((screen_size[0] - screen_size[1]) / 2.) as f64;
                            } else {
                                offset_x = ((screen_size[0] - screen_size[1]) / 2.) as f64;
                            }
                            let x = ((mouse_x + offset_y) / max_screen_size)  * (2. * zoom_factor) + center[0] - zoom_factor;
                            let y = ((mouse_y + offset_x) / max_screen_size) * (2. * zoom_factor) + center[1] - zoom_factor;
                            println!("Set for {}, {} as center for animation.", x, y);
                            anim_center = [x, y];
                            // center = anim_center;
                            // draw = true;
                        }
                        WindowEvent::KeyboardInput { input, .. } => match input {
                            KeyboardInput { virtual_keycode, state: ElementState::Pressed, .. } => { draw = true; match virtual_keycode {
                                Some(VirtualKeyCode::Escape) => closed = true,
                                Some(VirtualKeyCode::Up) => z[0] += pan_rate * 0.1,
                                Some(VirtualKeyCode::Down) => z[0] -= pan_rate * 0.1,
                                Some(VirtualKeyCode::Left) => z[1] += pan_rate * 0.1,
                                Some(VirtualKeyCode::Right) => z[1] -= pan_rate * 0.1,
                                Some(VirtualKeyCode::W) => center[1] += pan_rate,
                                Some(VirtualKeyCode::S) => center[1] -= pan_rate,
                                Some(VirtualKeyCode::D) => center[0] += pan_rate,
                                Some(VirtualKeyCode::A) => center[0] -= pan_rate,
                                Some(VirtualKeyCode::C) => {
                                    color_mode = (color_mode + 1) % 2
                                },
                                Some(VirtualKeyCode::T) => {
                                    max_iter += 1;
                                    println!("max_iter: {}", max_iter);
                                },
                                Some(VirtualKeyCode::G) => {
                                    if max_iter > 2 {
                                        max_iter -= 1;
                                    }
                                    println!("max_iter: {}", max_iter);
                                },
                                Some(VirtualKeyCode::E) => {
                                    zoom += zoom_rate;
                                    pan_rate = 0.5 / (zoom as f64).exp();
                                    println!("zoom: {}", zoom);
                                },
                                Some(VirtualKeyCode::Q) => {
                                    zoom -= zoom_rate;
                                    pan_rate = 0.5 / (zoom as f64).exp();
                                    println!("zoom: {}", zoom);
                                },
                                Some(VirtualKeyCode::Space) => {
                                    center = [0.0, 0.0]; zoom = f32::ln(1.0);
                                    pan_rate = 0.1; zoom_rate = 0.1; max_iter = 50;
                                    z = [0.0, 0.0]; color_mode = 0;
                                },
                                Some(VirtualKeyCode::Y) => {
                                    zoom_rate += 0.1;
                                    println!("zoom_rate: {}", zoom_rate);
                                },
                                Some(VirtualKeyCode::H) => {
                                    zoom_rate -= 0.1;
                                    println!("zoom_rate: {}", zoom_rate);
                                },
                                Some(VirtualKeyCode::M) => {
                                    if mode == 1 { mode = 0 } else { mode = 1};
                                },
                                Some(VirtualKeyCode::Z) => {
                                    anim = 0;
                                    max_iter = 5000;
                                    // center = anim_center;
                                    zoom = f32::ln(1.0);
                                    progress = ProgressBar::new((max_zoom / zoom_rate) as u64);
                                },
                                _ => (),
                            } }
                            _ => (),
                        }

                        _ => (),
                    },
                    _ => ()
                }
            });
        } else {
            events_loop.poll_events(|ev| {
                match ev {
                    Event::WindowEvent { event, .. } => match event {
                        WindowEvent::KeyboardInput { input, .. } => match input {
                            KeyboardInput { virtual_keycode, state: ElementState::Pressed, .. } => { draw = true; match virtual_keycode {
                                Some(VirtualKeyCode::Q) => {
                                    anim = -1;
                                    print!("Waiting on {} jobs ... ", pool.active_count());
                                    stdout().flush().unwrap();
                                    pool.join();
                                    println!("Done");
                                },
                                _ => ()
                            } },
                            _ => ()
                        },
                        _ => ()

                    },
                    _ => ()
                }
            });
        }
    }
}
