extern crate hlua;
extern crate sdl2;

use hlua::Lua;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::*;
use std::fs::File;
use std::io::prelude::*;
use std::time::Duration;

pub fn main() {
    let mut file = File::open("resources/main.lua").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Abbiocco", 512, 512)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut lua = Lua::new();
    lua.execute::<()>(&contents).unwrap();

    'running: loop {
        // Draw stuff

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
