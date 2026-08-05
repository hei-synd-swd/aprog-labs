use crate::entity::Entity;
use crate::position::Position;

pub struct Human {
  position: Position,
}

impl Human {
  pub fn new(position: Position) -> Self {
    Self { position }
  }

  fn nearest_zombie<'a>(&self, others: &'a [Box<dyn Entity>]) -> Option<&'a dyn Entity> {
    let my_pos = &self.position;
    let mut nearest: Option<&dyn Entity> = None;
    let mut nearest_dist = f64::MAX;

    for entity in others {
      if entity.is_human() {
        continue;
      }
      let dist = my_pos.distance(entity.position());
      if dist < nearest_dist {
        nearest_dist = dist;
        nearest = Some(entity.as_ref());
      }
    }

    nearest
  }
}

impl Entity for Human {
  fn position(&self) -> &Position {
    &self.position
  }

  fn set_position(&mut self, pos: Position) {
    self.position = pos;
  }

  fn calculate_new_position(&self, others: &[Box<dyn Entity>]) -> Option<Position> {
    let my_pos = self.position;

    let zombie = self.nearest_zombie(others)?;
    let zombie_pos = zombie.position();

    let dx = (my_pos.x() - zombie_pos.x()).signum();
    let dy = (my_pos.y() - zombie_pos.y()).signum();

    crate::utils::find_free_neighbor(&my_pos, dx, dy, others)
  }

  fn is_alive(&self) -> bool {
    true
  }

  fn is_human(&self) -> bool {
    true
  }

  fn as_str(&self) -> &str {
    "\x1B[1;32mH\x1B[0m"
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::zombie::Zombie;

  #[test]
  fn test_01_new() {
    let pos = Position::new(4, 5);
    let h = Human::new(pos);
    assert_eq!(h.position().x(), 4);
    assert_eq!(h.position().y(), 5);
  }

  #[test]
  fn test_02_set_position() {
    let mut h = Human::new(Position::new(0, 0));
    h.set_position(Position::new(7, 8));
    assert_eq!(h.position().x(), 7);
    assert_eq!(h.position().y(), 8);
  }

  #[test]
  fn test_03_move_away_from_zombie() {
    let h = Human::new(Position::new(5, 5));
    let z: Box<dyn Entity> = Box::new(Zombie::new(Position::new(5, 2)));
    let new_pos = h.calculate_new_position(&[z]);
    assert_eq!(new_pos.unwrap().x(), 5);
    assert_eq!(new_pos.unwrap().y(), 6);
  }

  #[test]
  fn test_04_no_move_without_zombies() {
    let h = Human::new(Position::new(5, 5));
    let new_pos = h.calculate_new_position(&[]);
    assert!(new_pos.is_none());
  }

  #[test]
  fn test_05_move_clamped_to_edge() {
    let h = Human::new(Position::new(0, 0));
    let z: Box<dyn Entity> = Box::new(Zombie::new(Position::new(0, 1)));
    let new_pos = h.calculate_new_position(&[z]);
    assert_eq!(new_pos.unwrap().x(), 1);
    assert_eq!(new_pos.unwrap().y(), 0);
  }

  #[test]
  fn test_06_move_nearest_zombie() {
    let h = Human::new(Position::new(5, 5));
    let z1: Box<dyn Entity> = Box::new(Zombie::new(Position::new(0, 0)));
    let z2: Box<dyn Entity> = Box::new(Zombie::new(Position::new(5, 2)));
    let new_pos = h.calculate_new_position(&[z1, z2]);
    assert_eq!(new_pos.unwrap().x(), 5);
    assert_eq!(new_pos.unwrap().y(), 6);
  }
}
