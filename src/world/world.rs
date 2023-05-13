use crate::characters::{ Damage, Alive };

pub struct World<T, U> {
    pub player: T,
    pub enemy: U,
}

impl <T: Damage + Alive, U: Damage + Alive> World<T, U> {

    pub fn fight(&mut self) {
        println!("FIGHT!!!");
        println!("YOUR HEALTH: {}", self.player.get_hp());
        println!("ENEMY'S HEALTH: {}", self.enemy.get_hp());
        let player_dmg = self.player.get_dmg();
        let enemy_dmg = self.enemy.get_dmg();
        while self.enemy.is_alive() && self.player.is_alive() {
            self.enemy.take_damage(player_dmg);
            if !self.enemy.is_alive() {
                println!("Player wins!");
                break
            }
            self.player.take_damage(enemy_dmg);
            if !self.player.is_alive() {
                println!("Enemy wins!");
                break
            }
        }
    }
}
