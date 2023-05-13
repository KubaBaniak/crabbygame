mod characters;
mod world;
mod sdl_config;

use characters::{ enemy::Enemy, player::Player };
use world::world::World;

fn main() {
    let mut world = World {
        player: Player::new(),
        enemy: Enemy::new()
    };
    world.fight();
    sdl_config::initalize_sdl();
}
