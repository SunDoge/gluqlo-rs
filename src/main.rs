use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::surface::{Surface, SurfaceRef};
// use std::time::{Duration, Instant};
// use time;
use sdl2::{ttf::Sdl2TtfContext, video::Window, EventPump, Sdl};
use std::fmt::Write;
use structopt::StructOpt;

const FONT: &'static str = "gluqlo.ttf";
const TITLE: &'static str = "Gluqlo 1.1";

const DEFAULT_A: u8 = 0xff;
const FONT_COLOR: Color = Color {
    r: 0xb7,
    g: 0xb7,
    b: 0xb7,
    a: DEFAULT_A,
};
const BACKGROUND_COLOR: Color = Color {
    r: 0x0f,
    g: 0x0f,
    b: 0x0f,
    a: DEFAULT_A,
};

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    #[structopt(short, long, default_value = "1024")]
    width: u32,

    #[structopt(short, long, default_value = "768")]
    height: u32,

    #[structopt(short = "s", long, default_value = "1.")]
    display_scale_factor: f32,
}

struct ScreenSaver {
    window: Window,
    event_pump: EventPump,
}

impl ScreenSaver {
    pub fn new(sdl_context: &Sdl, opt: &Opt) -> ScreenSaver {
        let mut width = opt.width;
        let mut height = opt.height;
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window(TITLE, width, height)
            .build()
            .unwrap();
        let event_pump = sdl_context.event_pump().unwrap();

        let (w, h) = window.size();
        width = (w as f32 * opt.display_scale_factor) as u32;
        height = (h as f32 * opt.display_scale_factor) as u32;

        let mut screen = window.surface(&event_pump).unwrap();
        screen.fill_rect(None, Color::RGB(0, 255, 255)).unwrap();
        screen.finish().unwrap();

        ScreenSaver { window, event_pump }
    }

    pub fn run(&mut self) {
        'running: loop {
            let mut receive_user_event = false;

            for event in self.event_pump.poll_iter() {
                match event {
                    Event::User {..} => receive_user_event = true,
                    Event::Quit { .. } => break 'running,
                    Event::KeyDown { keycode, .. } => match keycode {
                        Some(Keycode::Escape) | Some(Keycode::Q) => break 'running,
                        _ => {}
                    },
                    _ => {}
                }
            }

            if receive_user_event {
                self.render_animation();
            }

            // let mut screen = self.window.surface(&self.event_pump).unwrap();
            // screen.fill_rect(None, Color::RGB(0, 255, 255)).unwrap();
            // screen.finish().unwrap();
        }
    }

    fn fill_rounded_box_b(&mut self) {}

    fn render_ampm(&mut self) {}

    fn blit_digits(&mut self) {}

    fn render_digits(&mut self) {}

    fn render_clock(&mut self) {}

    fn render_animation(&mut self) {}

    fn update_time(&mut self) {}
}

fn main() -> Result<(), String> {
    let opt = Opt::from_args();

    println!("{:#?}", opt);

    let sdl_context = sdl2::init()?;

    let mut screen_saver = ScreenSaver::new(&sdl_context, &opt);

    screen_saver.run();

    Ok(())
}

fn set_pixels(pixels: &mut [u8], index: i32, pixcolor: u32) {
    unsafe {
        *(pixels.as_mut_ptr() as *mut u32).add(index as usize) = pixcolor;
    }
}

fn fill_rounded_box_b(dst: &mut SurfaceRef, coords: &Rect, r: i32, color: Color) {
    let pixcolor = color.to_u32(&dst.pixel_format());
    let rpsqrt2 = (r as f64 / 2.0_f64.sqrt()) as i32;
    let yd: i32 =
        (dst.pitch() as f32 / dst.pixel_format_enum().byte_size_per_pixel() as f32) as i32;
    let mut w: i32 = coords.width() as i32 / 2 - 1;
    let mut h: i32 = coords.height() as i32 / 2 - 1;
    let xo = coords.x() + w as i32;
    let yo = coords.y() + h as i32;

    w -= r;
    h -= r;

    if w <= 0 || h <= 0 {
        return;
    }

    dst.with_lock_mut(|pixels| {
        let sy: i32 = (yo - h) * yd;
        let ey: i32 = (yo + h) * yd;
        let sx: i32 = xo - w;
        let ex: i32 = xo + w;

        for i in (sy..=ey).step_by(yd as usize) {
            for j in (sx - r)..=(ex + r) {
                // let index = (i + j) as usize;
                // pixels[index + 0] = color.r;
                // pixels[index + 1] = color.g;
                // pixels[index + 2] = color.b;
                // pixels[index + 3] = color.a;

                // 如果我没理解错，就是一次赋4个值
                set_pixels(pixels, i + j, pixcolor);
            }
        }

        let mut d: i32 = -r;
        let mut x2m1: i32 = -1;
        let mut y: i32 = r;

        for x in 0..=rpsqrt2 {
            x2m1 += 2;
            d += x2m1;

            if d >= 0 {
                y -= 1;
                d -= y * 2;
            }

            for i in (sx - x)..=(ex + x) {
                set_pixels(pixels, sy - y * yd + i, pixcolor);
            }

            for i in (sx - y)..=(ex + y) {
                set_pixels(pixels, sy - x * yd + i, pixcolor);
            }

            for i in (sx - y)..=(ex + y) {
                set_pixels(pixels, ey + y * yd + i, pixcolor);
            }

            for i in (sx - x)..=(ex + x) {
                set_pixels(pixels, ey + x * yd + i, pixcolor);
            }
        }
    });
}