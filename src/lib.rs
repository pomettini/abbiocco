extern crate hlua;
extern crate sdl2;

pub mod colors;
pub mod drawing;
pub mod font;
pub mod math;
pub mod primitives;
pub mod sound;

use hlua::*;
use hlua::{AnyLuaValue, Lua};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use std::cell::RefCell;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::time::Duration;

use colors::*;
use drawing::*;
use font::*;
use math::*;
use primitives::*;
use sound::*;

static TARGET_FPS: u32 = 30;

/*
    TODO (by priority)
    * All graphical primitives
    * Input handling
    * Sprite support
    * Add fonts
    * Sfx
    * Music
    * Add tests lol
    * Error handling
*/

pub struct GameState {}

impl GameState {
    pub fn new(title: &str, width: usize, height: usize, path: &str) {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window(&title, width as u32, height as u32)
            .position_centered()
            .build()
            .unwrap();

        let canvas = RefCell::new(window.into_canvas().build().unwrap());
        let mut lua = Lua::new();

        let mut event_pump = sdl_context.event_pump().unwrap();

        init_drawing!(lua, canvas);
        init_math!(lua, canvas);
        init_primitives!(lua, canvas);
        init_sound!(lua, canvas);

        lua.execute_from_reader::<(), _>(File::open(&Path::new(&path)).unwrap())
            .unwrap();

        'running: loop {
            // Clear canvas

            canvas.borrow_mut().set_draw_color(colors::COLOR_BLACK);
            canvas.borrow_mut().clear();

            // Start draw stuff

            {
                let mut update_func: LuaFunction<_> = lua.get("_update").unwrap();
                let _update_ret: AnyLuaValue = update_func.call().unwrap();
            }

            {
                let mut draw_func: LuaFunction<_> = lua.get("_draw").unwrap();
                let _draw_ret: AnyLuaValue = draw_func.call().unwrap();
            }

            // let cur_frame: i32 = lua.get("cur_frame").unwrap();
            // println!("{:?}", cur_frame);

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

            canvas.borrow_mut().present();

            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / TARGET_FPS));
        }
    }
}
