use crate::entity::Entity;
use crate::position::Position;
use std::thread::sleep;
use std::time::Duration;
use std::time::Instant;

const INFECTION_THRESHOLD: usize = 1;

pub struct World {
    pub width: u32,
    pub height: u32,
    entities: Vec<Box<dyn Entity>>,
    pub turn: u32,
}

impl World {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            entities: Vec::new(),
            turn: 0,
        }
    }

    pub fn add_human(&mut self, human: crate::human::Human) {
        self.entities.push(Box::new(human));
    }

    pub fn add_zombie(&mut self, zombie: crate::zombie::Zombie) {
        self.entities.push(Box::new(zombie));
    }

    pub fn count_humans(&self) -> usize {
        self.entities.iter().filter(|e| e.is_human()).count()
    }

    pub fn count_zombies(&self) -> usize {
        self.entities.iter().filter(|e| !e.is_human()).count()
    }

    fn tick_movement(&mut self) -> Duration {
        let start = Instant::now();
        for i in 0..self.entities.len() {
            let new_pos = {
                let entity = &self.entities[i];
                entity.calculate_new_position(&self.entities)
            };
            if let Some(pos) = new_pos {
                self.entities[i].set_position(pos);
            }
        }
        start.elapsed()
    }

    fn tick_infection(&mut self) -> Duration {
        let start = Instant::now();
        let mut to_remove = Vec::new();
        let mut new_zombies = Vec::new();

        for (i, entity) in self.entities.iter().enumerate() {
            if !entity.is_human() {
                continue;
            }
            let pos = entity.position();
            let nearby_zombies = self.entities.iter().filter(|e| {
                if e.is_human() {
                    return false;
                }
                let dx = (e.position().x() - pos.x()).abs();
                let dy = (e.position().y() - pos.y()).abs();
                dx <= 1 && dy <= 1
            }).count();
            if nearby_zombies >= INFECTION_THRESHOLD {
                to_remove.push(i);
                new_zombies.push(crate::zombie::Zombie::new(*pos));
            }
        }

        for &i in to_remove.iter().rev() {
            self.entities.swap_remove(i);
        }
        for zombie in new_zombies {
            self.entities.push(Box::new(zombie));
        }
        start.elapsed()
    }

    fn tick_decay(&mut self) -> Duration {
        let start = Instant::now();
        for entity in &mut self.entities {
            entity.tick();
        }
        self.entities.retain(|e| e.is_alive());
        start.elapsed()
    }

    pub fn process_turn(&mut self) -> (Duration, Duration, Duration) {
        let d0 = self.tick_movement();
        let d1 = self.tick_infection();
        let d2 = self.tick_decay();
        self.turn += 1;
        (d0, d1, d2)
    }

    pub fn display(&self) {
        print!("\x1B[2J\x1B[H");

        for y in 0..self.height {
            for x in 0..self.width {
                let pos = Position::new(x as i32, y as i32);
                let mut found = false;
                for entity in &self.entities {
                    if entity.position() == &pos {
                        print!("{}", entity.as_str());
                        found = true;
                        break;
                    }
                }
                if !found {
                    print!("·");
                }
            }
            println!();
        }

        println!(
            "Tick {}  |  Humans: {:3}  |  Zombies: {:3}",
            self.turn,
            self.count_humans(),
            self.count_zombies(),
        );
    }

    pub fn run(&mut self) {
        loop {
            self.display();
            if self.count_humans() == 0 || self.count_zombies() == 0 {
                break;
            }
            self.process_turn();
            sleep(Duration::from_millis(250));
        }
        if self.count_humans() == 0 {
            println!("\x1B[1;31mZombies win!\x1B[0m");
        } else {
            println!("\x1B[1;32mHumans win!\x1B[0m");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::human::Human;
    use crate::zombie::Zombie;

    #[test]
    fn test_01_new() {
        let w = World::new(5, 5);
        assert_eq!(w.width, 5);
        assert_eq!(w.height, 5);
        assert!(w.entities.is_empty());
        assert_eq!(w.turn, 0);
    }

    #[test]
    fn test_02_add_human() {
        let mut w = World::new(10, 10);
        w.add_human(Human::new(Position::new(1, 2)));
        assert_eq!(w.count_humans(), 1);
    }

    #[test]
    fn test_03_add_zombie() {
        let mut w = World::new(10, 10);
        w.add_zombie(Zombie::new(Position::new(3, 4)));
        assert_eq!(w.count_zombies(), 1);
    }

    #[test]
    fn test_04_humans_and_zombies_filtered() {
        let mut w = World::new(10, 10);
        w.add_human(Human::new(Position::new(0, 0)));
        w.add_zombie(Zombie::new(Position::new(1, 1)));
        w.add_human(Human::new(Position::new(2, 2)));
        assert_eq!(w.count_humans(), 2);
        assert_eq!(w.count_zombies(), 1);
    }

    #[test]
    fn test_05_single_zombie_does_not_infect() {
        let mut w = World::new(10, 10);
        w.add_human(Human::new(Position::new(5, 5)));
        w.add_zombie(Zombie::new(Position::new(5, 2)));
        w.tick_movement();
        w.tick_infection();
        assert_eq!(w.count_humans(), 1);
    }

    #[test]
    fn test_06_two_zombies_infect_human() {
        let mut w = World::new(10, 10);
        w.add_zombie(Zombie::new(Position::new(0, 1)));
        w.add_zombie(Zombie::new(Position::new(1, 0)));
        w.add_human(Human::new(Position::new(0, 0)));
        w.process_turn();
        assert_eq!(w.count_humans(), 0);
    }

    #[test]
    fn test_07_process_turn_increments_turn() {
        let mut w = World::new(10, 10);
        w.add_human(Human::new(Position::new(0, 0)));
        w.add_zombie(Zombie::new(Position::new(9, 9)));
        assert_eq!(w.turn, 0);
        w.process_turn();
        assert_eq!(w.turn, 1);
    }

    #[test]
    fn test_08_zombie_dies_from_decay() {
        let mut w = World::new(10, 10);
        w.add_human(Human::new(Position::new(0, 0)));
        let mut zombie = Zombie::new(Position::new(9, 9));
        zombie.stamina = 15;
        w.add_zombie(zombie);
        w.process_turn();
        assert_eq!(w.count_zombies(), 1);
        w.process_turn();
        assert_eq!(w.count_zombies(), 0);
    }
}
