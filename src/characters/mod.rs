pub mod player;
pub mod enemy;

pub trait Damage {
    fn take_damage(&mut self, damage: u8);
    fn get_dmg(&self) -> u8;
}

pub trait Alive {
    fn is_alive(&self) -> bool;
    fn get_hp(&self) -> i8;
}
