use crate::human::Human;
use crate::position::Position;

pub const ZOMBIE_STAMINA: u32 = 100;

pub struct Zombie {
  position: Position,
  stamina: u32,
}

impl Zombie {
  pub fn new(position: Position) -> Self {
    todo!()
  }

  pub fn position(&self) -> &Position {
    todo!()
  }

  pub fn set_position(&mut self, position: Position) {
    todo!()
  }

  pub fn stamina(&self) -> u32 {
    todo!()
  }

  pub fn set_stamina(&mut self, stamina: u32) {
    todo!()
  }

  pub fn is_alive(&self) -> bool {
    todo!()
  }

  pub fn as_str(&self) -> &str {
    "\x1B[1;31mZ\x1B[0m"
  }

  pub fn calculate_new_position(&self, humans: &[Human], zombies: &[Zombie]) -> Option<Position> {
    let my_pos = self.position;

    let human = self.nearest_human(humans)?;
    let human_pos = human.position();

    let dx = (human_pos.x() - my_pos.x()).signum();
    let dy = (human_pos.y() - my_pos.y()).signum();

    let mid = crate::utils::find_free_neighbor_static(&my_pos, dx, dy, humans, zombies)?;
    let final_pos = crate::utils::find_free_neighbor_static(&mid, dx, dy, humans, zombies);

    final_pos.or(Some(mid))
  }

  fn nearest_human<'a>(&self, humans: &'a [Human]) -> Option<&'a Human> {
    let my_pos = &self.position;
    let mut nearest: Option<&Human> = None;
    let mut nearest_dist = f64::MAX;

    for human in humans {
      let dist = my_pos.distance(human.position());
      if dist < nearest_dist {
        nearest_dist = dist;
        nearest = Some(human);
      }
    }

    nearest
  }
}

#[cfg(all(test, feature = "part1"))]
mod tests {
  use super::*;
  use crate::human::Human;

  #[test]
  fn test_01_new() {
    let pos = Position::new(9, 9);
    let z = Zombie::new(pos);
    assert_eq!(z.position().x(), 9);
    assert_eq!(z.position().y(), 9);
    assert_eq!(z.stamina(), 100);
  }

  #[test]
  fn test_02_set_position() {
    let mut z = Zombie::new(Position::new(0, 0));
    z.set_position(Position::new(2, 3));
    assert_eq!(z.position().x(), 2);
    assert_eq!(z.position().y(), 3);
  }

  #[test]
  fn test_03_set_stamina() {
    let mut z = Zombie::new(Position::new(0, 0));
    z.set_stamina(30);
    assert_eq!(z.stamina(), 30);
  }

  #[test]
  fn test_04_is_alive() {
    let mut z = Zombie::new(Position::new(0, 0));
    assert!(z.is_alive());
    z.set_stamina(0);
    assert!(!z.is_alive());
  }

  #[test]
  fn test_05_move_toward_human() {
    let z_entity = Zombie::new(Position::new(5, 0));
    let h = Human::new(Position::new(5, 5));
    let new_pos = z_entity.calculate_new_position(&[h], &[]);
    assert_eq!(new_pos.unwrap().x(), 5);
    assert_eq!(new_pos.unwrap().y(), 2);
  }

  #[test]
  fn test_06_no_move_without_humans() {
    let z = Zombie::new(Position::new(5, 5));
    let new_pos = z.calculate_new_position(&[], &[]);
    assert!(new_pos.is_none());
  }

  #[test]
  fn test_07_move_two_cells() {
    let z_entity = Zombie::new(Position::new(5, 5));
    let h = Human::new(Position::new(5, 0));
    let new_pos = z_entity.calculate_new_position(&[h], &[]);
    assert_eq!(new_pos.unwrap().x(), 5);
    assert_eq!(new_pos.unwrap().y(), 3);
  }

  #[test]
  fn test_08_move_nearest_human() {
    let z_entity = Zombie::new(Position::new(5, 5));
    let h1 = Human::new(Position::new(0, 0));
    let h2 = Human::new(Position::new(5, 0));
    let new_pos = z_entity.calculate_new_position(&[h1, h2], &[]);
    assert_eq!(new_pos.unwrap().x(), 5);
    assert_eq!(new_pos.unwrap().y(), 3);
  }

  #[test]
  fn test_09_move_stops_at_human() {
    let z_entity = Zombie::new(Position::new(5, 4));
    let h = Human::new(Position::new(5, 5));
    let new_pos = z_entity.calculate_new_position(&[h], &[]);
    assert_eq!(new_pos.unwrap().x(), 6);
    assert_eq!(new_pos.unwrap().y(), 5);
  }
}

#[cfg(all(test, feature = "part2"))]
mod tests {
  use super::*;
  use crate::human::Human;

  #[test]
  fn test_01_new() {
    let pos = Position::new(9, 9);
    let z = Zombie::new(pos);
    assert_eq!(z.position().x(), 9);
    assert_eq!(z.position().y(), 9);
    assert_eq!(z.stamina, 100);
  }

  #[test]
  fn test_02_set_position() {
    let mut z = Zombie::new(Position::new(0, 0));
    z.set_position(Position::new(2, 3));
    assert_eq!(z.position().x(), 2);
    assert_eq!(z.position().y(), 3);
  }

  #[test]
  fn test_03_is_alive() {
    let mut z = Zombie::new(Position::new(0, 0));
    assert!(z.is_alive());
    z.stamina = 0;
    assert!(!z.is_alive());
  }

  #[test]
  fn test_04_tick() {
    let mut z = Zombie::new(Position::new(0, 0));
    z.tick();
    assert_eq!(z.stamina, 90);
  }

  #[test]
  fn test_05_tick_dies() {
    let mut z = Zombie::new(Position::new(0, 0));
    z.stamina = 5;
    z.tick();
    assert_eq!(z.stamina, 0);
    assert!(!z.is_alive());
  }

  #[test]
  fn test_06_move_toward_human() {
    let z_entity = Zombie::new(Position::new(5, 0));
    let h: Box<dyn Entity> = Box::new(Human::new(Position::new(5, 5)));
    let new_pos = z_entity.calculate_new_position(&[h]);
    assert_eq!(new_pos.unwrap().x(), 5);
    assert_eq!(new_pos.unwrap().y(), 2);
  }

  #[test]
  fn test_07_no_move_without_humans() {
    let z = Zombie::new(Position::new(5, 5));
    let new_pos = z.calculate_new_position(&[]);
    assert!(new_pos.is_none());
  }

  #[test]
  fn test_08_move_two_cells() {
    let z_entity = Zombie::new(Position::new(5, 5));
    let h: Box<dyn Entity> = Box::new(Human::new(Position::new(5, 0)));
    let new_pos = z_entity.calculate_new_position(&[h]);
    assert_eq!(new_pos.unwrap().x(), 5);
    assert_eq!(new_pos.unwrap().y(), 3);
  }

  #[test]
  fn test_09_move_nearest_human() {
    let z_entity = Zombie::new(Position::new(5, 5));
    let h1: Box<dyn Entity> = Box::new(Human::new(Position::new(0, 0)));
    let h2: Box<dyn Entity> = Box::new(Human::new(Position::new(5, 0)));
    let new_pos = z_entity.calculate_new_position(&[h1, h2]);
    assert_eq!(new_pos.unwrap().x(), 5);
    assert_eq!(new_pos.unwrap().y(), 3);
  }

  #[test]
  fn test_10_move_stops_at_human() {
    let z_entity = Zombie::new(Position::new(5, 4));
    let h: Box<dyn Entity> = Box::new(Human::new(Position::new(5, 5)));
    let new_pos = z_entity.calculate_new_position(&[h]);
    assert_eq!(new_pos.unwrap().x(), 6);
    assert_eq!(new_pos.unwrap().y(), 5);
  }
}
