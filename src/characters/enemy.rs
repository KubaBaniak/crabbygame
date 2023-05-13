use rand::Rng;
use crate::characters::Damage;
use crate::characters::Alive;

pub struct Enemy {
    hp: i8,
    crit: u8,
    dmg: u8,
}

impl Enemy {
    pub fn new() -> Self {
        Enemy {
            hp: 60,
            crit: 20,
            dmg: 15,
        }
    }

}

impl Damage for Enemy {
    fn take_damage(&mut self, mut damage: u8) {
        let num: u8 = rand::thread_rng().gen_range(0..100);
        if num <= self.crit {
            damage = 2*damage;
            self.hp -= 2*damage as i8;
        } else {
            self.hp -= damage as i8;
        }
        println!("You deal: {}", damage);
        println!("Enemy's Health left: {}", self.hp);
    }

    fn get_dmg(&self) -> u8 {
        self.dmg
    }
}

impl Alive for Enemy {
    fn is_alive(&self) -> bool {
        self.hp > 0
    } 
    fn get_hp(&self) -> i8 {
        self.hp
    }
}
