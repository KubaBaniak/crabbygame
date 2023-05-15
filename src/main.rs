mod characters;
mod world;
mod sdl_config;

use characters::{ enemy::Enemy, player::Player };
use world::world::World;

fn main() {
    sdl_config::initalize_sdl();
}
