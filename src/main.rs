extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;


const BLACK: Color = Color::RGB(0, 255, 255);
const SNAKE_COLOR: Color = Color::RGB(255, 0, 0);

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("crab-snake", 640, 480)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(BLACK);
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => {
                    println!("/\\");
                }
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    println!("<");
                }
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    println!("V");
                }
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    println!(">");
                }
                _ => {}
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
