use crate::entity::Entity;
use crate::position::Position;

pub const ZOMBIE_STAMINA: u32 = 100;
const ZOMBIE_DECAY: u32 = 10;

pub struct Zombie {
    position: Position,
    pub stamina: u32,
}

impl Zombie {
    pub fn new(position: Position) -> Self {
        Self {
            position,
            stamina: ZOMBIE_STAMINA,
        }
    }

    fn nearest_human<'a>(&self, others: &'a [Box<dyn Entity>]) -> Option<&'a dyn Entity> {
        let my_pos = &self.position;
        let mut nearest: Option<&dyn Entity> = None;
        let mut nearest_dist = f64::MAX;

        for entity in others {
            if !entity.is_human() {
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

impl Entity for Zombie {
    fn position(&self) -> &Position {
        &self.position
    }

    fn set_position(&mut self, pos: Position) {
        self.position = pos;
    }

    fn calculate_new_position(&self, others: &[Box<dyn Entity>]) -> Option<Position> {
        let my_pos = self.position;

        let human = self.nearest_human(others)?;
        let human_pos = human.position();

        let dx = (human_pos.x() - my_pos.x()).signum();
        let dy = (human_pos.y() - my_pos.y()).signum();

        let mid = crate::utils::find_free_neighbor(&my_pos, dx, dy, others)?;
        let final_pos = crate::utils::find_free_neighbor(&mid, dx, dy, others);

        final_pos.or(Some(mid))
    }

    fn tick(&mut self) {
        self.stamina = self.stamina.saturating_sub(ZOMBIE_DECAY);
    }

    fn is_alive(&self) -> bool {
        self.stamina > 0
    }

    fn is_human(&self) -> bool {
        false
    }

    fn as_str(&self) -> &str {
        "\x1B[1;31mZ\x1B[0m"
    }
}

#[cfg(test)]
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
