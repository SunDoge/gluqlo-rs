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
    hour_background: Rect,
    min_background: Rect,
    bgrect: Rect,
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

        ScreenSaver {
            window,
            event_pump,
            hour_background,
            min_background,
            bgrect,
        }
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
