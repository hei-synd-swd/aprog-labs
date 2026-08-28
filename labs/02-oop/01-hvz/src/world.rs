use crate::human::Human;
use crate::position::Position;
use crate::zombie::Zombie;
use std::thread::sleep;
use std::time::Duration;

const INFECTION_THRESHOLD: usize = 1;
const ZOMBIE_DECAY: u32 = 10;

pub struct World {
  pub width: u32,
  pub height: u32,
  pub humans: Vec<Human>,
  pub zombies: Vec<Zombie>,
  pub turn: u32,
}

impl World {
  pub fn new(width: u32, height: u32) -> Self {
    todo!()
  }

  pub fn add_human(&mut self, human: Human) {
    self.humans.push(human);
  }

  pub fn add_zombie(&mut self, zombie: Zombie) {
    self.zombies.push(zombie);
  }

  fn tick_movement(&mut self) {
    for i in 0..self.zombies.len() {
      let new_pos = {
        let zombie = &self.zombies[i];
        zombie.calculate_new_position(&self.humans, &self.zombies)
      };
      if let Some(pos) = new_pos {
        self.zombies[i].set_position(pos);
      }
    }
    for i in 0..self.humans.len() {
      let new_pos = {
        let human = &self.humans[i];
        human.calculate_new_position(&self.humans, &self.zombies)
      };
      if let Some(pos) = new_pos {
        self.humans[i].set_position(pos);
      }
    }
  }

  fn tick_infection(&mut self) {
    let mut infected: Vec<usize> = Vec::new();

    for (i, human) in self.humans.iter().enumerate() {
      let pos = human.position();
      let nearby = self
        .zombies
        .iter()
        .filter(|z| {
          let dx = (z.position().x() - pos.x()).abs();
          let dy = (z.position().y() - pos.y()).abs();
          dx <= 1 && dy <= 1
        })
        .count();
      if nearby >= INFECTION_THRESHOLD {
        infected.push(i);
      }
    }

    for &i in infected.iter().rev() {
      let pos = *self.humans[i].position();
      self.zombies.push(Zombie::new(pos));
      self.humans.swap_remove(i);
    }
  }

  fn tick_decay(&mut self) {
    todo!()
  }

  pub fn process_turn(&mut self) {
    self.tick_movement();
    self.tick_infection();
    self.tick_decay();
    self.turn += 1;
  }

  pub fn is_occupied(&self, pos: &Position) -> bool {
    self.humans.iter().any(|h| h.position() == pos)
      || self.zombies.iter().any(|z| z.position() == pos)
  }

  pub fn display(&self) {
    print!("\x1B[2J\x1B[H");

    print!("┌");
    for _ in 0..self.width {
      print!("─");
    }
    println!("┐");

    for y in 0..self.height {
      print!("│");
      for x in 0..self.width {
        let pos = Position::new(x as i32, y as i32);
        if self
          .humans
          .iter()
          .any(|h| h.position().x() == pos.x() && h.position().y() == pos.y())
        {
          print!("\x1B[1;32mH\x1B[0m");
        } else if self
          .zombies
          .iter()
          .any(|z| z.position().x() == pos.x() && z.position().y() == pos.y())
        {
          print!("\x1B[1;31mZ\x1B[0m");
        } else {
          print!("·");
        }
      }
      println!("│");
    }

    print!("└");
    for _ in 0..self.width {
      print!("─");
    }
    println!("┘");

    println!(
      "Tick {}  |  Humans: {:3}  |  Zombies: {:3}  |  Decay: -{}/tick  |  Infection: ≥{} adjacent",
      self.turn,
      self.humans.len(),
      self.zombies.len(),
      ZOMBIE_DECAY,
      INFECTION_THRESHOLD,
    );
  }

  pub fn run(&mut self) {
    loop {
      self.display();
      if self.humans.is_empty() || self.zombies.is_empty() {
        break;
      }
      self.process_turn();
      sleep(Duration::from_millis(250));
    }
    if self.humans.is_empty() {
      println!("\x1B[1;31mZombies win!\x1B[0m");
    } else {
      println!("\x1B[1;32mHumans win!\x1B[0m");
    }
  }
}

#[cfg(all(test, feature = "part1"))]
mod tests {
  use super::*;

  #[test]
  fn test_01_new() {
    let w = World::new(5, 5);
    assert_eq!(w.width, 5);
    assert_eq!(w.height, 5);
    assert!(w.humans.is_empty());
    assert!(w.zombies.is_empty());
    assert_eq!(w.turn, 0);
  }

  #[test]
  fn test_02_add_human() {
    let mut w = World::new(10, 10);
    w.add_human(Human::new(Position::new(1, 2)));
    assert_eq!(w.humans.len(), 1);
    assert_eq!(w.humans[0].position().x(), 1);
  }

  #[test]
  fn test_03_add_zombie() {
    let mut w = World::new(10, 10);
    w.add_zombie(Zombie::new(Position::new(3, 4)));
    assert_eq!(w.zombies.len(), 1);
    assert_eq!(w.zombies[0].position().x(), 3);
  }

  #[test]
  fn test_04_zombie_moves_toward_human() {
    let mut w = World::new(10, 10);
    w.add_human(Human::new(Position::new(5, 5)));
    w.add_zombie(Zombie::new(Position::new(5, 0)));
    w.tick_movement();
    assert_eq!(w.zombies[0].position().x(), 5);
    assert_eq!(w.zombies[0].position().y(), 2);
  }

  #[test]
  fn test_05_human_flees_from_zombie() {
    let mut w = World::new(10, 10);
    w.add_human(Human::new(Position::new(5, 5)));
    w.add_zombie(Zombie::new(Position::new(5, 2)));
    w.tick_movement();
    assert_eq!(w.humans[0].position().x(), 5);
    assert_eq!(w.humans[0].position().y(), 6);
  }

  #[test]
  fn test_06_single_zombie_does_not_infect() {
    let mut w = World::new(10, 10);
    w.add_human(Human::new(Position::new(5, 5)));
    w.add_zombie(Zombie::new(Position::new(5, 6)));
    w.tick_movement();
    w.tick_infection();
    assert_eq!(w.humans.len(), 1);
  }

  #[test]
  fn test_07_two_zombies_infect_human() {
    let mut w = World::new(10, 10);
    let mut z1 = Zombie::new(Position::new(0, 1));
    z1.set_stamina(200);
    w.add_zombie(z1);
    let mut z2 = Zombie::new(Position::new(1, 0));
    z2.set_stamina(200);
    w.add_zombie(z2);
    w.add_human(Human::new(Position::new(0, 0)));
    w.process_turn();
    assert!(w.humans.is_empty());
  }

  #[test]
  fn test_08_infection_does_not_remove_zombies() {
    let mut w = World::new(10, 10);
    let mut z1 = Zombie::new(Position::new(0, 1));
    z1.set_stamina(200);
    w.add_zombie(z1);
    let mut z2 = Zombie::new(Position::new(1, 0));
    z2.set_stamina(200);
    w.add_zombie(z2);
    w.add_human(Human::new(Position::new(0, 0)));
    w.process_turn();
    assert_eq!(w.zombies.len(), 3);
  }

  #[test]
  fn test_09_infection_evaluates_all_humans_from_snapshot() {
    let mut w = World::new(10, 10);
    let mut z = Zombie::new(Position::new(0, 0));
    z.set_stamina(200);
    w.add_zombie(z);
    w.add_human(Human::new(Position::new(1, 0)));
    w.add_human(Human::new(Position::new(2, 0)));
    w.tick_infection();
    assert_eq!(w.humans.len(), 1);
    assert_eq!(w.zombies.len(), 2);
  }

  #[test]
  fn test_10_zombie_loses_stamina_each_tick() {
    let mut w = World::new(10, 10);
    w.add_human(Human::new(Position::new(0, 0)));
    w.add_zombie(Zombie::new(Position::new(9, 9)));
    let initial_stamina = w.zombies[0].stamina();
    w.process_turn();
    assert!(w.zombies[0].stamina() < initial_stamina);
    assert_eq!(w.zombies[0].stamina(), initial_stamina - ZOMBIE_DECAY);
  }

  #[test]
  fn test_11_zombie_dies_from_decay() {
    let mut w = World::new(10, 10);
    w.add_human(Human::new(Position::new(0, 0)));
    let mut zombie = Zombie::new(Position::new(9, 9));
    zombie.set_stamina(15);
    w.add_zombie(zombie);
    w.process_turn();
    assert_eq!(w.zombies.len(), 1, "Zombie still alive after 1 tick");
    w.process_turn();
    assert!(w.zombies.is_empty(), "Zombie should die after 2 ticks");
  }

  #[test]
  fn test_12_process_turn_increments_turn() {
    let mut w = World::new(10, 10);
    w.add_human(Human::new(Position::new(0, 0)));
    w.add_zombie(Zombie::new(Position::new(9, 9)));
    assert_eq!(w.turn, 0);
    w.process_turn();
    assert_eq!(w.turn, 1);
    w.process_turn();
    assert_eq!(w.turn, 2);
  }
}

#[cfg(all(test, feature = "part2"))]
mod tests {
  use super::*;
  use crate::human::Human;
  use crate::robot::Robot;
  use crate::zombie::Zombie;

  #[test]
  fn test_robot_is_neutral_not_counted_as_zombie() {
    let mut w = World::new(10, 10);
    w.add_robot(Robot::new(Position::new(4, 4)));
    assert_eq!(w.count_robots(), 1);
    assert_eq!(w.count_zombies(), 0, "a robot must not count as a zombie");
    assert_eq!(w.count_humans(), 0);
  }

  #[test]
  fn test_robot_next_to_human_does_not_infect() {
    // A human adjacent only to a robot (no zombie) must NOT be infected.
    let mut w = World::new(10, 10);
    w.add_human(Human::new(Position::new(5, 5)));
    w.add_robot(Robot::new(Position::new(5, 6)));
    w.tick_infection();
    assert_eq!(w.count_humans(), 1, "a robot must not cause infection");
    assert_eq!(w.count_zombies(), 0);
    assert_eq!(w.count_robots(), 1, "the robot itself is never infected");
  }

  #[test]
  fn test_robot_survives_decay_ticks() {
    // Robots never lose stamina, so they are never removed by tick_decay.
    let mut w = World::new(10, 10);
    w.add_robot(Robot::new(Position::new(0, 0)));
    for _ in 0..100 {
      w.tick_decay();
    }
    assert_eq!(w.count_robots(), 1, "a robot must never decay away");
  }

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
