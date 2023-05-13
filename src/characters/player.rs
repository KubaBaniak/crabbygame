use rand::Rng;
use crate::characters::Damage;
use crate::characters::Alive;

pub struct Player {
    hp: i8,
    crit: u8,
    dmg: u8,
}

impl Player {
    pub fn new() -> Self {
        Player {
            hp: 100,
            crit: 60,
            dmg: 25,
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
