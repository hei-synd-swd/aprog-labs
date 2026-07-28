use std::collections::HashSet;

use rand::Rng;

use hvz::human::Human;
use hvz::position::Position;
use hvz::world::World;
use hvz::zombie::Zombie;

const WIDTH: u32 = 40;
const HEIGHT: u32 = 20;
const HUMAN_COUNT: usize = 30;
const ZOMBIE_COUNT: usize = 15;

fn random_positions(
    count: usize,
    occupied: &mut HashSet<(i32, i32)>,
    rng: &mut impl Rng,
) -> Vec<Position> {
    let mut positions = Vec::with_capacity(count);
    while positions.len() < count {
        let x = rng.gen_range(0..WIDTH as i32);
        let y = rng.gen_range(0..HEIGHT as i32);
        if occupied.insert((x, y)) {
            positions.push(Position::new(x, y));
        }
    }
    positions
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut occupied = HashSet::new();

    let human_positions = random_positions(HUMAN_COUNT, &mut occupied, &mut rng);
    let zombie_positions = random_positions(ZOMBIE_COUNT, &mut occupied, &mut rng);

    let mut world = World::new(WIDTH, HEIGHT);
    for pos in human_positions {
        world.add_human(Human::new(pos));
    }
    for pos in zombie_positions {
        world.add_zombie(Zombie::new(pos));
    }

    world.run();
}
