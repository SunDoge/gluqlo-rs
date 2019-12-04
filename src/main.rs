use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::surface::{Surface, SurfaceRef};
// use std::time::{Duration, Instant};
// use time;
use sdl2::{ttf::Sdl2TtfContext, ttf::Font, video::Window, Sdl, EventPump, render::WindowCanvas};
use std::fmt::Write;

const FONT: &'static str = "gluqlo.ttf";
const TITLE: &'static str = "Gluqlo 1.1";
const DEFAULT_WIDTH: u32 = 1024;
const DEFAULT_HEIGHT: u32 = 768;

const FONT_COLOR: Color = Color {
    r: 0xb7,
    g: 0xb7,
    b: 0xb7,
    a: 0xff,
};
const BACKGROUND_COLOR: Color = Color {
    r: 0x0f,
    g: 0x0f,
    b: 0x0f,
    a: 0xff,
};

struct ScreenSaver<'a> {
    screen: WindowCanvas,
    bg: Surface<'a>,
    bgrect: Rect,
    hour_background: Rect,
    min_background: Rect,
    font_time: Font<'a, 'a>,
    font_mode: Font<'a, 'a>,
    past_h: i32,
    past_m: i32,
    width: u32,
    height: u32,
    display_scale_factor: f32,
    twentyfourh: bool,
    leadingzero: bool,
    fullscreen: bool,
    animate: bool,
    rectsize: u32,
    spacing: i32,
    radius: i32,
    event_pump: EventPump,
}

impl<'a> ScreenSaver<'a> {
    pub fn new(sdl_context: &'a Sdl, ttf_context: &'a Sdl2TtfContext) -> Result<ScreenSaver<'a>, String> {
        let mut width = DEFAULT_WIDTH;
        let mut height = DEFAULT_HEIGHT;
        let mut display_scale_factor = 1.;

        let video_subsystem = sdl_context.video()?;
        let window = video_subsystem
            .window(TITLE, width, height)
            .build()
            .unwrap();

        let (w, h) = window.size();
        width = (w as f32 * display_scale_factor) as u32;
        height = (h as f32 * display_scale_factor) as u32;

        let font_time = ttf_context
            .load_font(FONT, (height as f32 / 1.68) as u16)
            .unwrap();
        let font_mode = ttf_context
            .load_font(FONT, (height as f32 / 16.5) as u16)
            .unwrap();

        let mut screen = window.into_canvas().build().unwrap();
        screen.set_draw_color(Color::RGB(0, 0, 0));
        screen.fill_rect(None)?;

        let rectsize = (height as f32 * 0.6) as u32;
        let spacing = (width as f32 * 0.031) as i32;
        let radius = (height as f32 * 0.05714) as i32;

        let mut jitter_width: i32 = 1;
        let mut jitter_height: i32 = 1;

        if display_scale_factor != 1. {
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
        let bg = Surface::new(rectsize, rectsize, PixelFormatEnum::RGBA32)?;

        let event_pump = sdl_context.event_pump()?;
    
        Ok(ScreenSaver {
            screen,
            bg,
            bgrect,
            hour_background,
            min_background,
            font_time,
            font_mode,
            width: DEFAULT_WIDTH,
            height: DEFAULT_HEIGHT,
            past_h: -1,
            past_m: -1,
            display_scale_factor: 1.,
            twentyfourh: true,
            leadingzero: false,
            fullscreen: false,
            animate: true,
            rectsize,
            spacing,
            radius,
            event_pump
        })
    }

    pub fn run(&mut self) -> Result<(), String> {

        // let ttf_context = sdl2::ttf::init().unwrap();
        // let font_time = ttf_context
        //     .load_font(FONT, (self.height as f32 / 1.68) as u16)
        //     .unwrap();
        // let font_mode = ttf_context
        //     .load_font(FONT, (self.height as f32 / 16.5) as u16)
        //     .unwrap();

        // let mut canvas = window.into_canvas().build().unwrap();
        // canvas.set_draw_color(Color::RGB(0, 0, 0));
        // canvas.fill_rect(None)?;

        self.fill_rounded_box_b(BACKGROUND_COLOR);
        self.render_clock(20, 19);

        // let mut event_pump = sdl_context.event_pump()?;

        'running: loop {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'running,
                    Event::KeyDown { keycode, .. } => match keycode {
                        Some(Keycode::Escape) | Some(Keycode::Q) => break 'running,
                        _ => {}
                    },
                    _ => {}
                }
            }
        }

        Ok(())
    }

    fn fill_rounded_box_b(&mut self, color: Color) {
        let dst = &mut self.bg;
        let coords = &self.bgrect;
        let r = self.radius;

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

    fn render_clock(&mut self, maxsteps: i32, step: i32) {
        let mut buffer = String::with_capacity(3);
        let mut buffer2 = String::with_capacity(3);

        let tm = time::now();

        if tm.tm_hour != self.past_h {
            let h = if self.twentyfourh {
                tm.tm_hour
            } else {
                (tm.tm_hour + 11) % 12 + 1
            };

            if self.leadingzero {
                write!(buffer, "{:02}", h);
                write!(buffer2, "{:02}", self.past_h);
            } else {
                write!(buffer, "{}", h);
                write!(buffer2, "{}", self.past_h);
            }
        }
    }

    fn render_digits(&mut self) {}
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    // let video_subsystem = sdl_context.video()?;
    // let screen = video_subsystem
    //     .window(TITLE, DEFAULT_WIDTH, DEFAULT_HEIGHT)
    //     .build()
    //     .unwrap();
    // let rectsize = 100;

    // let bgrect = Rect::new(0, 0, rectsize, rectsize);
    // let mut bg = Surface::new(rectsize, rectsize, PixelFormatEnum::RGBA32)?;
    let ttf_context = sdl2::ttf::init().unwrap();

    let mut screen_saver = ScreenSaver::new(&sdl_context, &ttf_context)?;
    screen_saver.run()
}

fn _main() -> Result<(), String> {
    // println!("linked sdl2_ttf: {}", sdl2::ttf::get_linked_version());
    let mut width = DEFAULT_WIDTH;
    let mut height = DEFAULT_HEIGHT;
    let mut display_scale_factor = 1.;

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(TITLE, width, height)
        .build()
        .unwrap();

    let (w, h) = window.size();
    width = (w as f32 * display_scale_factor) as u32;
    height = (h as f32 * display_scale_factor) as u32;

    let ttf_context = sdl2::ttf::init().unwrap();
    let font_time = ttf_context
        .load_font(FONT, (height as f32 / 1.68) as u16)
        .unwrap();
    let font_mode = ttf_context
        .load_font(FONT, (height as f32 / 16.5) as u16)
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.fill_rect(None)?;

    let rectsize = (height as f32 * 0.6) as u32;
    let spacing = (width as f32 * 0.031) as i32;
    let radius = (height as f32 * 0.05714) as i32;

    let mut jitter_width: i32 = 1;
    let mut jitter_height: i32 = 1;

    if display_scale_factor != 1. {
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

    let mut bg = Surface::new(rectsize, rectsize, PixelFormatEnum::RGBA32)?;
    fill_rounded_box_b(&mut bg, &bgrect, radius, BACKGROUND_COLOR);

    let timer_subsystem = sdl_context.timer()?;
    // let timer = timer_subsystem.add_timer(
    //     60,

    // )

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown { keycode, .. } => match keycode {
                    Some(Keycode::Escape) | Some(Keycode::Q) => break 'running,
                    _ => {}
                },
                _ => {}
            }
        }
    }

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

fn render_clock(maxsteps: i32, step: i32) {
    // let time = Instant::now();
    let tm = time::now();
}
