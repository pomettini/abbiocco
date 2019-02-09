extern crate hlua;
extern crate sdl2;

use hlua::{AnyLuaValue, Lua};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::time::Duration;

static TARGET_FPS: u32 = 30;

pub fn main() {
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

    fn rectfill_func(x0: u32, y0: u32, x1: u32, y1: u32) {
        // canvas.fill_rect(Rect::new(10, 10, 780, 580));
        println!("{:?}", x0);
    }

    lua.set("rectfill", hlua::function4(rectfill_func));

    lua.execute_from_reader::<(), _>(File::open(&Path::new("resources/main.lua")).unwrap())
        .unwrap();

    'running: loop {
        // Start draw stuff

        {
            let mut update_func: hlua::LuaFunction<_> = lua.get("_update").unwrap();
            let update_ret: hlua::AnyLuaValue = update_func.call().unwrap();
        }

        {
            let mut draw_func: hlua::LuaFunction<_> = lua.get("_draw").unwrap();
            let draw_ret: hlua::AnyLuaValue = draw_func.call().unwrap();
        }

        let cur_frame: i32 = lua.get("cur_frame").unwrap();
        println!("{:?}", cur_frame);

        // End draw stuff

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

        // canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / TARGET_FPS));
    }
}
