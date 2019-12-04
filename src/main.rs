use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::surface::{Surface, SurfaceRef};
// use std::time::{Duration, Instant};
// use time;
use sdl2::{ttf::Sdl2TtfContext, video::Window, Sdl, EventPump};
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

    #[structopt(short, long, default_value="1024")]
    width: u32,

    #[structopt(short, long, default_value="768")]
    height: u32,
}


struct ScreenSaver {
    window: Window,
    event_pump: EventPump,
}

impl ScreenSaver {
    pub fn new(sdl_context: &Sdl, opt: &Opt) -> ScreenSaver {
        let mut width = opt.width;
        let mut heigth = opt.height;
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window(TITLE, width, heigth)
            .build().unwrap();

        let event_pump = sdl_context.event_pump().unwrap();

        ScreenSaver { window, event_pump }
    }

    pub fn run(&mut self) {
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
