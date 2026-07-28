use crate::position::Position;
use crate::entity::Entity;

pub fn find_free_neighbor(
    pos: &Position,
    pref_dx: i32,
    pref_dy: i32,
    entities: &[Box<dyn Entity>],
) -> Option<Position> {
    let width = 40;
    let height = 20;
    let clampx = |x: i32, d: i32| (x + d).clamp(0, width as i32 - 1);
    let clampy = |y: i32, d: i32| (y + d).clamp(0, height as i32 - 1);

    let is_occupied = |p: &Position| -> bool {
        entities.iter().any(|e| e.position() == p)
    };

    let step_dx = pref_dx.signum();
    let step_dy = pref_dy.signum();

    let candidates = [
        (pref_dx, pref_dy),
        (step_dx, step_dy),
        (step_dx, 0),
        (0, step_dy),
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];
    for &(dx, dy) in &candidates {
        let nx = clampx(pos.x(), dx);
        let ny = clampy(pos.y(), dy);
        let candidate = Position::new(nx, ny);
        if !is_occupied(&candidate) && candidate != *pos {
            return Some(candidate);
        }
    }
    None
}
