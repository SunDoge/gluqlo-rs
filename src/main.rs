use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::surface::{Surface, SurfaceRef};
// use std::time::{Duration, Instant};
// use time;
use sdl2::{
    ttf::Font, ttf::Sdl2TtfContext, video::Window, video::WindowSurfaceRef, EventPump, Sdl,
};
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

    #[structopt(long = "ampm")]
    ampm: bool,

    #[structopt(long = "leardingzero")]
    leadingzero: bool,
}

struct ScreenSaver<'a> {
    window: Window,
    event_pump: EventPump,
    hour_background: Rect,
    min_background: Rect,
    bgrect: Rect,
    bg: Surface<'a>,
    font_time: Font<'a, 'a>,
    font_mode: Font<'a, 'a>,
    opt: &'a Opt,
    past_h: i32,
    past_m: i32,
}

impl<'a> ScreenSaver<'a> {
    pub fn new(
        sdl_context: &Sdl,
        ttf_context: &'a Sdl2TtfContext,
        opt: &'a Opt,
    ) -> ScreenSaver<'a> {
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

        let font_time = ttf_context
            .load_font(FONT, (height as f32 / 1.68) as u16)
            .unwrap();
        let font_mode = ttf_context
            .load_font(FONT, (height as f32 / 16.5) as u16)
            .unwrap();

        let mut screen = window.surface(&event_pump).unwrap();
        screen.fill_rect(None, Color::RGB(0, 255, 255)).unwrap();
        screen.finish().unwrap();

        let rectsize = (height as f32 * 0.6) as u32;
        let spacing = (width as f32 * 0.031) as i32;
        let radius = (height as f32 * 0.05714) as i32;

        let mut jitter_width: i32 = 1;
        let mut jitter_height: i32 = 1;

        if opt.display_scale_factor != 1. {
            jitter_width = ((w - width) as f32 * 0.5) as i32;
            jitter_height = ((h - height) as f32 * 0.5) as i32;
        }

        let hour_background = Rect::new(
            (0.5 * (width as f32 - (0.031 * width as f32) - (1.2 * height as f32))) as i32
                + jitter_width,
            (0.2 * height as f32) as i32 + jitter_height,
            rectsize,
            rectsize,
        );

        let min_background = Rect::new(
            hour_background.x() + (0.6 * height as f32) as i32 + spacing,
            hour_background.y(),
            rectsize,
            rectsize,
        );

        let bgrect = Rect::new(0, 0, rectsize, rectsize);

        // dbg!(PixelFormatEnum::RGB24.into_masks());
        let mut bg = Surface::new(rectsize, rectsize, PixelFormatEnum::RGBA32).unwrap();
        fill_rounded_box_b(&mut bg, &bgrect, radius, BACKGROUND_COLOR);

        ScreenSaver {
            window,
            event_pump,
            hour_background,
            min_background,
            bgrect,
            bg,
            font_time,
            font_mode,
            opt,
            past_h: -1,
            past_m: -1,
        }
    }

    pub fn run(&mut self) {
        self.render_clock(20, 19);

        'running: loop {
            let mut receive_user_event = false;

            for event in self.event_pump.poll_iter() {
                match event {
                    Event::User { .. } => receive_user_event = true,
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

    // fn fill_rounded_box_b(&mut self) {}

    fn render_ampm(&self, surface: &mut SurfaceRef, rect: &Rect, pm: bool) {
        let mode = format!("{}M", if pm { "P" } else { "A" });

        let ampm = self.font_mode.render(&mode).blended(FONT_COLOR).unwrap();

        let offset = (rect.height() as f32 * 0.127) as i32;
        let coords = Rect::new(
            rect.x() + (rect.height() as f32 * 0.07) as i32,
            rect.y()
                + if pm {
                    rect.height() as i32 - offset - ampm.height() as i32
                } else {
                    offset
                },
            0,
            0,
        );
        // surface.blit(src_rect: R1, dst: &mut SurfaceRef, dst_rect: R2)
        ampm.blit(None, surface, coords);
    }

    fn blit_digits(&self, surface: &mut SurfaceRef, rect: &Rect, spc: i32, digits: &str, color: Color) {
        let adjust_x = if digits.starts_with("1") {
            (2.5 * spc as f32) as i32
        } else {
            0
        };

        if digits.len() > 2 {
            // self.font_time.find_glyph_metrics(ch: char)
        } else {

        }
    }

    fn render_digits(
        &self,
        surface: &mut SurfaceRef,
        background: &Rect,
        digits: &str,
        prevdigits: &str,
        maxsteps: i32,
        step: i32,
    ) {
        let spc = surface.height();

        let rect = Rect::new(
            background.x(),
            background.y(),
            background.width(),
            background.height() / 2,
        );
        surface.set_clip_rect(rect);
        self.bg.blit(None, surface, rect);
        // self.blit_digits();
        surface.set_clip_rect(None);
    }

    fn render_clock(&mut self, maxsteps: i32, step: i32) {
        let mut buffer = String::with_capacity(3);
        let mut buffer2 = String::with_capacity(3);

        let tm = time::now();
        let mut screen = self.window.surface(&self.event_pump).unwrap();

        if tm.tm_hour != self.past_h {
            let h = if self.opt.ampm {
                (tm.tm_hour + 11) % 12 + 1
            } else {
                tm.tm_hour
            };

            if self.opt.leadingzero {
                write!(buffer, "{:02}", h);
                write!(buffer2, "{:02}", self.past_h);
            } else {
                write!(buffer, "{}", h);
                write!(buffer2, "{}", self.past_h);
            }

            self.render_digits(&mut screen, &self.hour_background, &buffer, &buffer2, maxsteps, step);
            if self.opt.ampm {
                self.render_ampm(&mut screen, &self.hour_background, tm.tm_hour >= 12);
            }
        }

        screen.finish().unwrap();
    }

    fn render_animation(&mut self) {}

    fn update_time(&mut self) {}
}

fn main() -> Result<(), String> {
    let opt = Opt::from_args();

    println!("{:#?}", opt);

    let sdl_context = sdl2::init()?;
    let ttf_context = sdl2::ttf::init().unwrap();

    let mut screen_saver = ScreenSaver::new(&sdl_context, &ttf_context, &opt);

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
