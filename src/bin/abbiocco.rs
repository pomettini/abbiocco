extern crate abbiocco;
extern crate clap;

use abbiocco::*;

pub fn main() {
    GameState::new("Abbiocco", 512, 512, "resources/main.lua");
}
