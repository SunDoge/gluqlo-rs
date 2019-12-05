use sdl2::event::{Event, EventType};
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::surface::{Surface, SurfaceRef};
// use std::time::{Duration, Instant};
// use time;
use sdl2::{
    gfx::rotozoom::RotozoomSurface, ttf::Font, ttf::Sdl2TtfContext, video::Window,
    video::WindowSurfaceRef, EventPump, Sdl, TimerSubsystem, EventSubsystem,
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
    // radius: i32,
    animate: bool,
    time_subsystem: TimerSubsystem,
    event_subsystem: EventSubsystem,
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
        screen.fill_rect(None, Color::RGB(0, 0, 0)).unwrap();
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

        let time_subsystem = sdl_context.timer().unwrap();
        let event_subsystem = sdl_context.event().unwrap();

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
            // radius,
            animate: true,
            time_subsystem,
            event_subsystem,
        }
    }

    pub fn run(&mut self) {
        self.render_clock(20, 19);
        let past_m = self.past_m;
        let event_subsystem = &self.event_subsystem;
        self.time_subsystem.add_timer(60, Box::new(move || {
            let time_i = time::now();

            let interval = if time_i.tm_min != past_m {
                let e = Event::User {
                    type_: EventType::User as u32,
                    code: 0,
                    data1: std::ptr::null_mut(),
                    data2: std::ptr::null_mut(),
                    window_id: 0,
                    timestamp: 0,
                };
                event_subsystem.push_event(e);
                (1000 * (60 - time_i.tm_sec) - 250) as u32
            } else {
                250
            };
            interval
        }));

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

            // fill_rounded_box_b(&mut self.bg, &self.bgrect, self.radius, BACKGROUND_COLOR);

            // self.render_clock(20, 19);
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

    fn blit_digits(
        &self,
        surface: &mut SurfaceRef,
        rect: Rect,
        spc: i32,
        digits: &str,
        color: Color,
    ) {
        let digits = digits.as_bytes();

        let adjust_x = if digits[0] == b'1' {
            (2.5 * spc as f32) as i32
        } else {
            0
        };
        let center_x = rect.x() + rect.width() as i32 / 2 - adjust_x;

        if digits.len() > 2 {
            let glyph_metrics = self
                .font_time
                .find_glyph_metrics(digits[0] as char)
                .unwrap();
            let glyph = self
                .font_time
                .render(&digits[0].to_string())
                .blended(color)
                .unwrap();
            let coords = Rect::new(
                center_x - glyph_metrics.maxx + glyph_metrics.minx
                    - spc
                    - (if adjust_x > 0 { spc } else { 0 }),
                rect.y() + (rect.height() as i32 - glyph.height() as i32) / 2,
                0,
                0,
            );
            glyph.blit(None, surface, coords);

            let glyph_metrics = self
                .font_time
                .find_glyph_metrics(digits[1] as char)
                .unwrap();
            let glyph = self
                .font_time
                .render(&digits[1].to_string())
                .blended(color)
                .unwrap();
            let coords = Rect::new(
                center_x + spc / 2,
                rect.y() + (rect.height() as i32 - glyph.height() as i32) / 2,
                0,
                0,
            );
            glyph.blit(None, surface, coords);
        } else {
            let glyph = self
                .font_time
                .render(&digits[0].to_string())
                .blended(color)
                .unwrap();
            let coords = Rect::new(
                center_x - glyph.width() as i32 / 2,
                rect.y() + (rect.height() as i32 - glyph.height() as i32) / 2,
                0,
                0,
            );
            glyph.blit(None, surface, coords);
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
        let spc = surface.height() as i32;

        let rect = Rect::new(
            background.x(),
            background.y(),
            background.width(),
            background.height() / 2,
        );
        surface.set_clip_rect(rect);
        self.bg.blit(None, surface, rect);
        self.blit_digits(surface, rect, spc, digits, FONT_COLOR);
        surface.set_clip_rect(None);

        let halfsteps = maxsteps / 2;
        let upperhalf = (step + 1) <= halfsteps;
        let scale = if upperhalf {
            1.0 - step as f64 / (halfsteps as f64 - 1.)
        } else {
            (step as f64 - halfsteps as f64 + 1.) / halfsteps as f64
        };

        let c = if upperhalf {
            0xb7 - 0xb7 * (step as f32 / (halfsteps as f32 - 1.)) as u8
        } else {
            0xb7 * ((step as f32 - halfsteps as f32 + 1.) / halfsteps as f32) as u8
        };

        let color = Color::RGB(c, c, c);

        let mut bgcopy = self.bg.convert(&self.bg.pixel_format()).unwrap();

        let rect = Rect::new(0, 0, bgcopy.width(), bgcopy.height());

        self.blit_digits(
            &mut bgcopy,
            rect,
            spc,
            if upperhalf { prevdigits } else { digits },
            color,
        );

        let scaled = bgcopy.zoom(1., scale, true).unwrap();
        // rect.set_x(0);
        // rect.set_y(if upperhalf {
        //     0
        // } else {
        //     scaled.height() as i32 / 2
        // });
        // rect.set_width(scaled.width());
        // rect.set_height(scaled.height() / 2);
        let rect = Rect::new(
            0,
            if upperhalf {
                0
            } else {
                scaled.height() as i32 / 2
            },
            scaled.width(),
            scaled.height() / 2,
        );
        let dstrect = Rect::new(
            background.x(),
            background.y()
                + if upperhalf {
                (background.height() as i32 - scaled.height() as i32) / 2
            } else {
                background.height() as i32 / 2
            },
            rect.width(),
            rect.height(),
        );
        surface.set_clip_rect(dstrect);
        scaled.blit(rect, surface, dstrect);
        surface.set_clip_rect(None);

        if !self.animate {
            return;
        }

        let mut rect = Rect::new(
            background.x(),
            background.y() + (background.height() as i32 - rect.height() as i32) / 2,
            (surface.height() as f32 * 0.005) as u32,
            background.width(),
        );
        surface.fill_rect(rect, Color::RGB(0, 0, 0));
        rect.set_y(rect.y() + rect.height() as i32);
        rect.set_height(1);
        surface.fill_rect(rect, Color::RGB(0x1a, 0x1a, 0x1a));
    }

    fn render_clock(&mut self, maxsteps: i32, step: i32) {
        let mut buffer = String::with_capacity(3);
        let mut buffer2 = String::with_capacity(3);
        // let mut buffer: Vec<u8> = Vec::with_capacity(3);
        // let mut buffer2: Vec<u8> = Vec::with_capacity(3);

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

            self.render_digits(
                &mut screen,
                &self.hour_background,
                &buffer,
                &buffer2,
                maxsteps,
                step,
            );
            if self.opt.ampm {
                self.render_ampm(&mut screen, &self.hour_background, tm.tm_hour >= 12);
            }
        }

        if tm.tm_min != self.past_m {
            write!(buffer, "{:02}", tm.tm_min);
            write!(buffer2, "{:02}", self.past_m);
            self.render_digits(&mut screen, &self.min_background, &buffer, &buffer2, maxsteps, step);
        }

        screen.finish().unwrap();

        if step == maxsteps - 1 {
            self.past_h = tm.tm_hour;
            self.past_m = tm.tm_min;
        }
    }

    fn render_animation(&mut self) {
        if !self.animate {
            self.render_clock(20, 19);
            return;
        }

        const DURATION: u32 = 260;
        let start_tick = self.time_subsystem.ticks();
        let end_tick = start_tick + DURATION;

        let mut done = false;
        while !done {
            let mut current_tick = self.time_subsystem.ticks();
            if current_tick >= end_tick {
                done = true;
                current_tick = end_tick;
            }
            let frame = 99 * (current_tick - start_tick) / (end_tick - start_tick);
            self.render_clock(100, frame as i32);
        }
    }

    fn update_time(&mut self) -> u32 {
        let time_i = time::now();

        let interval = if time_i.tm_min != self.past_m {
//            let e = Event::User {
//                timestamp: 0,
//                code: 0,
//                data1: std::ptr::null_mut(),
//                data2: std::ptr::null_mut(),
//                type_: EventType::User as u32,
//            };
//            let e = Event::User::default();

            (1000 * (60 - time_i.tm_sec) - 250) as u32
        } else {
            250
        };
        interval
    }
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


