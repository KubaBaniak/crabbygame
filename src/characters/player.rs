use rand::Rng;
use crate::characters::{ Damage, Alive };
use sdl2::rect::{ Point, Rect };

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    K,
    J,
    H,
    L,
}

pub struct Player {
    hp: i8,
    crit: u8,
    dmg: u8,
    pub position: Point,
    pub sprite: Rect,
    pub speed: i32,
    pub direction: Direction,
    pub current_frame: i32,
}

impl Player {
    pub fn new(point: Point, rect: Rect) -> Self {
        Player {
            hp: 100,
            crit: 60,
            dmg: 25,
            position: point,
            sprite: rect,
            speed: 0,
            direction: Direction::H,
            current_frame: 0,
        }
    }
}

impl Damage for Player {
    fn take_damage(&mut self, mut damage: u8) {
        let num: u8 = rand::thread_rng().gen_range(0..100);
        if num <= self.crit {
            damage = 2*damage;
            self.hp -= damage as i8;
        } else {
            self.hp -= damage as i8;
        }
        println!("Enemy deals: {}", damage);
        println!("Player's Health left: {}", self.hp);
    }

    fn get_dmg(&self) -> u8 {
        self.dmg
    }
}
impl Alive for Player {
    fn is_alive(&self) -> bool {
        self.hp > 0
    } 
    fn get_hp(&self) -> i8 {
        self.hp
    }
}
