use crate::position::Position;
pub trait Entity {
  fn position(&self) -> &Position;
  fn set_position(&mut self, pos: Position);
  fn calculate_new_position(&self, others: &[Box<dyn Entity>]) -> Option<Position>;
  fn tick(&mut self) {}
  fn is_alive(&self) -> bool;
  fn is_human(&self) -> bool;
  fn as_str(&self) -> &str;
}
