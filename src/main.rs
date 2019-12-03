use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::surface::Surface;

const FONT: &'static str = "gluqlo.ttf";
const TITLE: &'static str = "Gluqlo 1.1";
const DEFAULT_WIDTH: u32 = 1024;
const DEFAULT_HEIGHT: u32 = 768;

fn main() -> Result<(), String> {
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
    let radius = (height as f32 * 0.05714) as u32;

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
